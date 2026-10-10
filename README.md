# 🗓️ rust1 — Calendario en la terminal

Un calendario de consola escrito en **Rust** desde cero: muestra un mes, cambia de mes con el teclado, marca el día de hoy y tiene pruebas automáticas.

> Este README es también el **cuaderno de la sesión**. Recoge todo lo que se explicó —conceptos de Rust, el *toolchain*, el terminal, git— y los problemas del sistema que fueron apareciendo, con su solución.

## Índice

1. [Qué hace](#1-qué-hace)
2. [Cómo ejecutarlo](#2-cómo-ejecutarlo)
3. [Cómo se usa](#3-cómo-se-usa)
4. [Estructura del proyecto](#4-estructura-del-proyecto)
5. [El código, función por función](#5-el-código-función-por-función)
6. [Conceptos de Rust de esta sesión](#6-conceptos-de-rust-de-esta-sesión)
7. [Las pruebas automáticas](#7-las-pruebas-automáticas)
8. [El toolchain](#8-el-toolchain)
9. [El terminal: por qué hacía falta Enter](#9-el-terminal-por-qué-hacía-falta-enter)
10. [Historia del proyecto: las 7 versiones](#10-historia-del-proyecto-las-7-versiones)
11. [Problemas del entorno y cómo se resolvieron](#11-problemas-del-entorno-y-cómo-se-resolvieron)
12. [Git](#12-git)
13. [Chuleta de comandos](#13-chuleta-de-comandos)
14. [Glosario](#14-glosario)
15. [Ideas para seguir](#15-ideas-para-seguir)

---

## 1. Qué hace

- Pide el **año** y el **mes**.
- Dibuja el mes en cuadrícula, con la semana empezando en **lunes**.
- Marca con un `*` el **día de hoy**.
- Cambia de mes **al instante** con `n`/`→` y `p`/`←`; sale con `q`.
- Limpia la pantalla antes de cada mes (un mes por pantalla).
- Tiene **5 pruebas automáticas**.

Ejemplo real de salida (octubre de 2026, día 10):

```
    OCTUBRE 2026
 Lu Ma Mi Ju Vi Sá Do
           1  2  3  4
  5  6  7  8  9*10 11
 12 13 14 15 16 17 18
 19 20 21 22 23 24 25
 26 27 28 29 30 31
```

---

## 2. Cómo ejecutarlo

Requisitos: Rust instalado (mira [El toolchain](#8-el-toolchain)).

```powershell
cd G:\repos\rust1

cargo run               # compila y ejecuta el calendario
cargo test              # ejecuta las pruebas automáticas
cargo build             # solo compila; deja el .exe en target\debug\
cargo build --release   # compilación optimizada, en target\release\
```

Versiones de este equipo: **cargo 1.99.0**, **rustc 1.99.0**, edición **2024**.

---

## 3. Cómo se usa

| Tecla | Efecto |
|---|---|
| `n` o `→` (flecha derecha) | mes siguiente |
| `p` o `←` (flecha izquierda) | mes anterior |
| `q` | salir |
| cualquier otra | se ignora y vuelve a dibujar |

Al arrancar, el año y el mes se escriben **y se confirman con Enter** (son números de varias cifras, ahí Enter es lo natural). La navegación entre meses, en cambio, es **instantánea**: no hace falta Enter.

---

## 4. Estructura del proyecto

| Archivo | Para qué sirve |
|---|---|
| `Cargo.toml` | la ficha del proyecto: nombre, versión, edición y **dependencias** |
| `Cargo.lock` | las versiones exactas de cada dependencia; lo escribe cargo, no se toca a mano |
| `src/main.rs` | todo el código del programa y sus pruebas |
| `.gitignore` | lo que git no debe seguir: `/target` (lo compilado) |
| `target/` | resultado de la compilación (no se sube a git) |

Dependencias actuales:

```toml
[dependencies]
console = "0.15"
time = { version = "0.3", features = ["local-offset"] }
```

---

## 5. El código, función por función

### `action_for` — traducir la tecla a una acción

```rust
#[derive(Debug, PartialEq)]
enum Action {
    Next,
    Previous,
    Quit,
    Nothing,
}

fn action_for(key: Key) -> Action {
    match key {
        Key::Char('n') | Key::Char('N') | Key::ArrowRight => Action::Next,
        Key::Char('p') | Key::Char('P') | Key::ArrowLeft => Action::Previous,
        Key::Char('q') | Key::Char('Q') => Action::Quit,
        _ => Action::Nothing,
    }
}
```

Convierte lo que se pulsa en una decisión. El `|` se lee **"o"**: esa rama se activa con `n`, con `N` **o** con la flecha derecha. Lo que no aparece cae en `_` y no hace nada.

Esta función es la clave del diseño: **la decisión está separada de la lectura del teclado**, y por eso se puede probar sin teclado.

### `read_line` — leer una línea de texto

```rust
fn read_line(prompt: &str) -> String {
    println!("{}", prompt);

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("no he podido leer del teclado");

    line.trim().to_string()
}
```

- `prompt: &str`: recibe un texto prestado (el mensaje que se muestra).
- `String::new()` crea un texto vacío; `mut` porque va a cambiar.
- `read_line(&mut line)` **escribe dentro** de esa caja prestada.
- `trim()` quita espacios y el salto de línea final; `to_string()` lo copia a un `String` nuevo.
- `.expect(...)`: si la lectura falla, el programa avisa y se detiene.

### `read_number` — pedir un número sin rendirse

```rust
fn read_number(prompt: &str) -> i32 {
    loop {
        match read_line(prompt).parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Eso no es un número. Prueba otra vez."),
        }
    }
}
```

`loop` repite hasta que se sale con `return`. `parse::<i32>()` intenta convertir el texto en número y devuelve un `Result`: `Ok(número)` o `Err`. Si es `Err`, se avisa y se vuelve a preguntar.

### `month_name` — el nombre del mes en español

```rust
fn month_name(month: Month) -> &'static str {
    match month {
        Month::January => "ENERO",
        Month::February => "FEBRERO",
        // ... hasta DICIEMBRE
    }
}
```

Un `match` sobre las 12 variantes de `Month`. No hay rama `_` porque están cubiertas todas: si añadieras un mes, el compilador te obligaría a contemplarlo.

### `month_as_text` — construir el calendario como texto

```rust
fn month_as_text(year: i32, month: Month, today: (i32, Month, u8)) -> String {
    let days_in_month = month.length(year);
    let first_day = Date::from_calendar_date(year, month, 1).expect("fecha no válida");
    let blank_cells = first_day.weekday().number_days_from_monday();

    let mut text = String::new();

    let title = format!("{} {}", month_name(month), year);
    text.push_str(&format!("{:^21}\n", title));
    text.push_str(" Lu Ma Mi Ju Vi Sá Do\n");

    let mut column = 0;
    for _ in 0..blank_cells {
        text.push_str("   ");
        column += 1;
    }

    for day in 1..=days_in_month {
        if year == today.0 && month == today.1 && day == today.2 {
            text.push_str(&format!("*{:>2}", day));
        } else {
            text.push_str(&format!("{:>3}", day));
        }

        column += 1;
        if column == 7 {
            text.push('\n');
            column = 0;
        }
    }

    if column != 0 {
        text.push('\n');
    }

    text
}
```

Puntos clave:

- **Devuelve** el texto en vez de imprimirlo. Una función que imprime no se puede comparar; una que devuelve texto, sí. Así nació la posibilidad de probarla.
- `month.length(year)` ya sabe cuántos días tiene cada mes, **años bisiestos incluidos**.
- `blank_cells` son los huecos antes del día 1: lunes = 0, martes = 1, ... domingo = 6.
- El truco de la cuadrícula: un contador `column` que, al llegar a 7, salta de línea y vuelve a 0.
- El día de hoy se pinta con `*{:>2}` en lugar de `{:>3}`: el asterisco ocupa el hueco de la izquierda y **los números siguen alineados**.

### `main` — el programa

```rust
fn main() {
    let term = Term::stdout();

    let now = OffsetDateTime::now_local().expect("no he podido obtener la fecha local");
    let today = (now.year(), now.month(), now.day());

    println!("Calendario. Hoy es {}.", now.date());
    let mut year = read_number("Año: ");
    let month_number = read_number("Mes (1 a 12): ");

    if month_number < 1 || month_number > 12 {
        println!("El mes tiene que estar entre 1 y 12.");
        return;
    }
    let mut month = Month::try_from(month_number as u8).expect("mes fuera de rango");

    loop {
        print!("\x1b[2J\x1b[H");
        print!("{}", month_as_text(year, month, today));
        println!();
        println!("n o -> (flecha derecha) = mes siguiente");
        println!("p o <- (flecha izquierda) = mes anterior");
        println!("q = salir");

        let key = term.read_key().expect("no he podido leer el teclado");

        match action_for(key) {
            Action::Next => {
                month = month.next();
                if month == Month::January {
                    year += 1;
                }
            }
            Action::Previous => {
                month = month.previous();
                if month == Month::December {
                    year -= 1;
                }
            }
            Action::Quit => break,
            Action::Nothing => {}
        }
    }

    println!("¡Hasta luego!");
}
```

- `print!("\x1b[2J\x1b[H")` son **códigos de control del terminal**: borrar la pantalla y llevar el cursor arriba.
- El cambio de año está a mano: de diciembre se pasa a enero del año siguiente, y al revés.
- `term.read_key()` espera **una** tecla (ver [sección 9](#9-el-terminal-por-qué-hacía-falta-enter)).

---

## 6. Conceptos de Rust de esta sesión

### Variables y tipos

```rust
let month_name = "ENERO";   // texto (&str)
let year = 2026;            // número entero
let mut column = 0;         // "mut" = esta variable cambiará después
const OTRO_DIA: (i32, Month, u8) = (1999, Month::January, 1);  // constante
```

Rust **deduce** el tipo del valor, no hay que escribirlo. Sin `mut`, cambiar una variable es un error de compilación: eso es Rust protegiéndote.

Tipos que han aparecido: `i32` (entero con signo), `u32`, `u8` (enteros sin signo), `i64`, `&str` (texto prestado), `String` (texto propio), `bool`.

### Funciones

```rust
/// Documentación de la función (los tres /// son un comentario de documentación).
fn read_number(prompt: &str) -> i32 {
    // ...
}
```

Entre paréntesis van los **parámetros** y después de `->` el **tipo que devuelve**. Si no devuelve nada, se omite el `->`. La última expresión sin punto y coma es el valor devuelto.

### Imprimir y dar formato

```rust
println!("{}", prompt);          // escribe y salta de línea
print!("   ");                   // escribe sin saltar
let title = format!("{} {}", month_name(month), year);   // construye un texto
```

| Formato | Significado |
|---|---|
| `{}` | un hueco que se rellena con el valor |
| `{:>3}` | hueco de 3 caracteres, alineado a la **derecha** (`  1`, ` 10`, ` 31`) |
| `{:^21}` | hueco de 21 caracteres, **centrado** |
| `{day:>2}` | nombre del valor dentro del hueco |

### Bucles

```rust
for day in 1..=days_in_month { }   // 1, 2, 3 ... hasta el último (..= incluye el final)
for _ in 0..blank_cells { }        // 0, 1, 2 ... (.. no incluye el final); "_" = no uso el número
loop { }                           // repite hasta que un "break" lo detenga
```

### Condicionales

```rust
if month_number < 1 || month_number > 12 { }   // || se lee "o"
if column == 7 { }
```

### `match`

Es el "if de varias opciones", y el compilador comprueba que no se te olvide ninguna:

```rust
match key {
    Key::Char('n') => Action::Next,
    Key::Char('q') => Action::Quit,
    _ => Action::Nothing,          // "_" recoge todo lo demás
}
```

Se ha usado para números (el mes), para texto (las órdenes de navegación) y para enumerados (las teclas y las acciones).

### Enum: una lista cerrada de posibilidades

```rust
#[derive(Debug, PartialEq)]
enum Action {
    Next,
    Previous,
    Quit,
    Nothing,
}
```

`enum` define un valor que **solo** puede ser una de esas opciones. `#[derive(Debug, PartialEq)]` pide a Rust que genere automáticamente el código para poder imprimirlo (`Debug`) y para poder compararlo con `==` (`PartialEq`).

### Tuplas: varios valores en uno

```rust
let today = (now.year(), now.month(), now.day());   // (i32, Month, u8)
today.0   // año
today.1   // mes
today.2   // día
```

### `String` y `&str`, y el préstamo `&mut`

- `&str` es texto **prestado** (no es tuyo, solo lo miras).
- `String` es texto **propio** (puede crecer y cambiar).
- `&mut line` significa "te presto esta caja para que escribas dentro". Así el valor rellenado sigue siendo tuyo cuando la función termina.

Métodos usados: `String::new()`, `push_str(...)` (añadir texto), `push('\n')` (añadir un carácter), `trim()` (quitar espacios y saltos), `trim_start()`, `to_string()`, `lines()` (recorrer las líneas de un texto), `len()`.

### `Option` y `Result`: fallar sin petar

```rust
match line.trim().parse::<i32>() {
    Ok(number) => return number,
    Err(_) => println!("Eso no es un número."),
}

let first_day = Date::from_calendar_date(year, month, 1).expect("fecha no válida");
```

- `Result<T, E>` es `Ok(valor)` si salió bien o `Err(error)` si salió mal.
- `Option<T>` es `Some(valor)` o `None` (no hay valor).
- `.expect("mensaje")` es la vía rápida: si no hay valor, el programa se detiene con ese mensaje. Sirve mientras aprendes; en un programa serio se trata el error.

### Conversión de tipos

```rust
month as u8                          // conversión directa (cuidado con los negativos)
Month::try_from(month_number as u8)  // conversión comprobada: devuelve Result
u8::from(now.month())
```

### Módulos y pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;   // trae las funciones del archivo de arriba
    // ...
}
```

`mod tests` crea un "cajón" de código dentro del mismo archivo. El `#[cfg(test)]` hace que **solo** se compile al ejecutar `cargo test`: el programa final no lo lleva.

### Dependencias

```toml
[dependencies]
console = "0.15"
time = { version = "0.3", features = ["local-offset"] }
```

Declarar una dependencia y ejecutar cargo la descarga, la compila y la deja disponible con `use`. `features` activa partes opcionales de la librería (aquí, la que sabe la hora local).

---

## 7. Las pruebas automáticas

### Qué son y para qué sirven

Una prueba es código que **comprueba** que otra parte del programa hace lo que debe. Sirven para dos cosas: detectar errores hoy y **poder cambiar cosas mañana sin miedo**, porque si rompes algo, cargo te avisa.

### Cómo se escribe una prueba

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn february_2024_has_29_days() {
        let text = month_as_text(2024, Month::February, OTRO_DIA);
        let last_week = text.lines().last().unwrap().trim();
        assert_eq!(last_week, "26 27 28 29");
    }
}
```

- `#[test]` marca la función como prueba; cargo las encuentra solo.
- `assert_eq!(a, b)` comprueba que dos cosas son iguales.
- `assert!(condición, "mensaje")` comprueba que algo es verdad; el mensaje se ve cuando falla.
- `unwrap()` y `expect(...)` se usan aquí sin problema: si algo va mal en una prueba, quieres que falle ruidosamente.

### Las 5 pruebas del proyecto

| Prueba | Qué comprueba |
|---|---|
| `january_2026_starts_on_thursday` | que enero de 2026 empieza en jueves (3 huecos) y la primera semana es `1  2  3  4` |
| `february_2024_has_29_days` | que un año bisiesto tiene 29 días |
| `february_2025_has_28_days` | que un año normal tiene 28 |
| `today_gets_a_star` | que el 10 de octubre de 2026 sale marcado (`9*10`) |
| `keys_map_to_actions` | que las teclas y las flechas producen la acción correcta |

### Ejecutarlas

```powershell
cargo test                     # todas
cargo test february            # solo las que contengan "february" en el nombre
```

Resultado esperado:

```
running 5 tests
test tests::january_2026_starts_on_thursday ... ok
test tests::february_2024_has_29_days ... ok
test tests::february_2025_has_28_days ... ok
test tests::today_gets_a_star ... ok
test tests::keys_map_to_actions ... ok

test result: ok. 5 passed; 0 failed
```

### La prueba que falló: una lección

La primera vez, `january_2026_starts_on_thursday` falló: la prueba esperaba **9** espacios delante del día 1 y había **11**. El programa estaba bien; **la expectativa estaba mal** (3 huecos de 3 caracteres son 9 espacios, más los 2 con los que se alinea el propio número). Se corrigió la expectativa, no el código.

Esa es la esencia de una prueba: fija por escrito lo que crees que debe pasar y te obliga a decidir, cuando falla, si el error está en el programa o en tu idea.

### Compruébalo tú

Cambia en `src/main.rs` `number_days_from_monday()` por `number_days_from_sunday()`, guarda y ejecuta `cargo test`. Verás el aviso con lo esperado y lo encontrado. Después deshazlo.

---

## 8. El toolchain

**Toolchain** significa literalmente *cadena de herramientas*: el conjunto de programas que convierten el texto que escribes en un programa que funciona. No es *una* herramienta, son varias **encadenadas**.

### Las piezas

| Pieza | Qué hace | Dónde está |
|---|---|---|
| `cargo` | El **jefe de obra**. No compila: organiza. Lee `Cargo.toml`, descarga dependencias, llama al compilador con los argumentos correctos, ejecuta, lanza las pruebas | `~/.cargo/bin/cargo` |
| `rustc` | El **traductor**. Convierte Rust en instrucciones de máquina y comprueba tipos y préstamos | dentro del toolchain |
| El **enlazador** (*linker*) | El que **emplata**: une tus trozos compilados con las piezas del sistema y produce el `.exe` | `link.exe` (MSVC) o `gcc`/`ld` (GNU) |
| Herramientas de apoyo | `as` (ensamblador), `ar`, `dlltool`... las pide cada librería cuando las necesita | las trae el toolchain, a veces a medias |
| `rustup` | **No es el toolchain**: es quien lo instala y decide cuál está activo | `~/.cargo/bin/rustup` |

Por eso siempre se escribe `cargo run` y nunca `rustc` directamente: cargo sabe en qué orden llamar a las demás.

### Los "sabores"

```
stable-x86_64-pc-windows-gnu
│      │      │  │       └─ enlaza con las herramientas de GNU (MinGW)
│      │      │  └───────── sistema operativo: Windows
│      │      └──────────── genérico (no un móvil ni una consola)
│      └─────────────────── procesador de 64 bits
└────────────────────────── canal estable (existen nightly y beta)
```

En este equipo conviven dos: `stable-x86_64-pc-windows-msvc` y `stable-x86_64-pc-windows-gnu`. **El activo es el GNU.**

### Ver qué tienes instalado

```powershell
rustup show              # toolchains instalados y cuál está activo
rustup toolchain list    # la lista
rustup default <nombre>  # cambiar el activo
rustc --version
cargo --version
```

### Por qué este equipo usa GNU

MSVC era el candidato natural (es la opción estándar en Windows), pero Visual Studio está instalado **sin el componente de C++**, así que no había enlazador: `link.exe not found`. Se instaló el toolchain GNU (`rustup toolchain install stable-x86_64-pc-windows-gnu`), que trae su propio enlazador dentro del toolchain. Ver [problemas del entorno](#11-problemas-del-entorno-y-cómo-se-resolvieron).

---

## 9. El terminal: por qué hacía falta Enter

El terminal, por defecto, trabaja **por líneas** (se llama *cooked mode*): el sistema acumula lo que tecleas y no se lo entrega al programa hasta que pulsas Enter. Por eso `io::stdin().read_line(...)` no puede reaccionar a una tecla suelta: para cuando recibe algo, ya has pulsado Enter.

Para leer **teclas sueltas** hay que preguntar al sistema por cada evento de teclado. Eso es lo que hace la librería `console` (la misma que usa el propio Cargo):

```rust
let term = Term::stdout();
let key = term.read_key().expect("no he podido leer el teclado");
```

`read_key()` espera **una** tecla y la devuelve enseguida. Cada tecla llega como un valor de `console::Key`: `Key::Char('n')`, `Key::ArrowRight`, `Key::Enter`...

Por eso los números del principio siguen usando `read_line` (teclear "2026" son varias teclas y hay que confirmar cuándo terminas) y la navegación usa `read_key`.

Y por eso el código tiene esta forma: **leer** (`read_key`) y **decidir** (`action_for`) están separados. La decisión se prueba; la lectura se confía al terminal.

Detalle del sistema de archivos: `console` funciona con el toolchain GNU porque usa `windows-sys` (librerías precompiladas), no la técnica `raw-dylib` que fue la que dio problemas (ver la sección siguiente).

---

## 10. Historia del proyecto: las 7 versiones

El proyecto se construyó paso a paso, cada versión en un commit.

| Versión | Commit | Qué se añadió | Qué se aprendió |
|---|---|---|---|
| 1 | `a561196` | Un mes fijo (ENERO 2026) escrito a mano | `println!`, formato, bucles, variables |
| 2 | `178178e` | El mes se calcula solo, dentro de una función | funciones, `match`, primera dependencia |
| 3 | `eccecff` | Pregunta el año y el mes por teclado | `String`, `&mut`, `parse`, `Result`, `loop` |
| 4 | `7dab51f` | Navegar con `n`/`p`/`q` | `loop` con `break`, `match` con texto, estado mutable |
| 5 | `7f2a815` | Marca el día de hoy y migra de `chrono` a `time` | dependencias y sus costes; verificar una migración |
| 6 | `e0ad842` | Limpia la pantalla entre meses | códigos de control del terminal |
| 7 | `5c719d0`, `58434ce`, `a819286`, `e7a8652` | Pruebas, teclas instantáneas y flechas | pruebas, `enum`, lectura de teclas |

El primer commit, `3730a88`, es el andamio inicial (lo que hace `cargo init`).

---

## 11. Problemas del entorno y cómo se resolvieron

Tres cosas fallaron por el *entorno*, no por el código. Quedan aquí documentadas por si vuelven.

### 11.1 El sandbox no podía escribir en la carpeta del proyecto

**Síntoma:** la primera orden de la sesión murió antes de ejecutar nada:

```
SetNamedSecurityInfoW failed (Win32 5): grantWrite(G:\repos\rust1)
```

**Causa:** a la carpeta `G:\repos\rust1` le faltaba el permiso de **cambio de propietario** para el usuario: tenía control total, pero heredado, sin una entrada propia.

**Solución:** se ejecutó una reparación de permisos que añadió una entrada de control total para el usuario en esa carpeta. Se verificó después con escrituras reales. El informe y el script de reversión quedaron en `G:\repos\dsh-acl-recovery` (fuera del repositorio).

**Nota relacionada:** con la política `workspace-write`, el sandbox solo puede escribir lo que él mismo crea; los archivos creados antes con acceso total (este proyecto entero) le están vedados. Por eso este proyecto se trabaja con la política de **acceso total**.

### 11.2 No había enlazador: link.exe no aparece

**Síntoma:**

```
error: linker `link.exe` not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
```

**Causa:** Visual Studio estaba instalado (Community y BuildTools 18) pero **sin el componente "Desarrollo para escritorio con C++"**: `vswhere` no encontraba las herramientas MSVC en ninguna instalación.

**Solución adoptada:** instalar el toolchain GNU, que incluye su propio enlazador:

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile default
rustup default stable-x86_64-pc-windows-gnu
```

**Alternativa (el camino estándar):** instalar ese componente de C++ desde el Instalador de Visual Studio y volver con `rustup default stable-x86_64-pc-windows-msvc`.

### 11.3 chrono necesitaba dlltool y as

**Síntoma** al usar `chrono` con su parte de reloj local:

```
error: error calling dlltool 'dlltool.exe': program not found
```

y después, con `dlltool` ya localizado:

```
error: dlltool could not create import library with ...\dlltool.exe ...
       ...\dlltool.exe: CreateProcess
```

**Causa:** `chrono` usa una técnica de Windows llamada `raw-dylib` (enlazar DLLs sin ficheros de importación), que en el sabor GNU exige el paquete completo de *binutils*: `dlltool` **y** el ensamblador `as`. El toolchain de Rust trae `dlltool.exe` en `...\x86_64-pc-windows-gnu\bin\self-contained\`, pero **no** trae `as`, así que `dlltool` arrancaba y fallaba con ese mensaje tan poco informativo.

**Solución adoptada:** cambiar la librería de fechas a `time`, que no usa esa técnica y da la fecha local sin necesitar nada extra. La migración se verificó comparando la salida del programa antes y después (idéntica) y con pruebas.

**Otras salidas posibles:** instalar MinGW-w64 completo (aporta `dlltool` y `as`), o instalar el C++ de Visual Studio y volver a MSVC.

---

## 12. Git

### El repositorio

- Remoto: `https://github.com/jrep41/rust1`
- Rama: `main`
- Todo el trabajo está subido y sincronizado.

### Comandos usados

```powershell
git status --short --branch   # qué ha cambiado y en qué rama estoy
git diff                      # ver los cambios línea a línea
git log --oneline             # historial resumido
git add -A                    # preparar todos los cambios
git commit -m "mensaje"       # guardar el punto de control
git push                      # subir al repositorio remoto
```

### Flujo

Se hizo **un commit por avance**, con el mensaje en inglés y en imperativo: `feat:` para funcionalidad nueva, `test:` para pruebas, `chore:` para limpieza, `docs:` para documentación.

### Avisos de git que aparecen en Windows

```
warning: in the working copy of 'src/main.rs', LF will be replaced by CRLF the next time Git touches it
```

No es un error: significa que el archivo usa **LF** (salto de línea de Linux) y Windows usa **CRLF**. Git lo normaliza. Se puede ignorar tranquilamente.

---

## 13. Chuleta de comandos

| Para... | Comando |
|---|---|
| Ejecutar el calendario | `cargo run` |
| Ejecutar las pruebas | `cargo test` |
| Compilar sin ejecutar | `cargo build` |
| Ver la versión de Rust | `rustc --version` / `cargo --version` |
| Ver los toolchains | `rustup toolchain list` / `rustup show` |
| Cambiar de toolchain | `rustup default <nombre>` |
| Añadir una librería | escribirla en `[dependencies]` de `Cargo.toml` |
| Ver qué ha cambiado | `git status` / `git diff` |
| Guardar un avance | `git add -A` y `git commit -m "..."` |
| Subir a GitHub | `git push` |
| Descartar cambios de un archivo | `git checkout -- <archivo>` |

---

## 14. Glosario

| Término | Significado |
|---|---|
| **crate** | Una librería de Rust (lo que se declara en `[dependencies]`), o el propio proyecto |
| **cargo** | La herramienta que organiza y ejecuta la compilación |
| **rustc** | El compilador de Rust |
| **toolchain** | El conjunto de herramientas: compilador, enlazador y apoyo |
| **enlazador / linker** | El programa que une todo y produce el `.exe` |
| **raw-dylib** | Técnica de Windows para enlazar DLLs sin ficheros de importación; en GNU exige `dlltool` y `as` |
| **dependencia** | Librería externa que usa tu proyecto |
| **compilar** | Traducir el código a un programa ejecutable |
| **módulo** | Un cajón de código dentro de un archivo (`mod`) |
| **enum** | Un valor que solo puede ser una de varias opciones |
| **prueba / test** | Código que comprueba que el programa hace lo que debe |
| **target** | La combinación de sistema y herramientas para la que compilas |
| **cooked mode** | Modo del terminal en el que la entrada llega por líneas (hay que pulsar Enter) |
| **workspace** | La carpeta de trabajo del proyecto |

---

## 15. Ideas para seguir

- **Colorear el día de hoy** con códigos ANSI (`\x1b[7m` ... `\x1b[0m`) en lugar del asterisco.
- **Ver el año entero** de una vez (12 meses en tres columnas).
- **Resaltar los fines de semana** (las columnas de sábado y domingo).
- **Guardar meses favoritos** en un archivo: enseña a leer y escribir ficheros.
- **Más pruebas**: que el día 1 nunca cae en la columna equivocada, para una lista de meses conocidos.
- **Volver a MSVC** instalando el componente C++ de Visual Studio, y comprobar que todo sigue funcionando.

---

*Proyecto hecho paso a paso en una sola sesión. Todas las versiones están en el historial de git.*
