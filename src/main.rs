// Versión 7: las teclas n, p y q actúan al momento, sin pulsar Enter.
// Para leer una tecla suelta usamos la librería console.
// (El terminal normal entrega el texto por líneas: hasta que no pulsas Enter
// no nos llega nada. console pregunta directamente al sistema por cada tecla.)

use console::{Key, Term};
use std::io;
use time::{Date, Month, OffsetDateTime};

/// Qué queremos hacer según la tecla que se pulse.
#[derive(Debug, PartialEq)]
enum Action {
    Next,
    Previous,
    Quit,
    Nothing,
}

/// Traduce una tecla a una acción. Esta parte se puede probar sin teclado.
fn action_for(key: Key) -> Action {
    match key {
        Key::Char('n') | Key::Char('N') | Key::ArrowRight => Action::Next,
        Key::Char('p') | Key::Char('P') | Key::ArrowLeft => Action::Previous,
        Key::Char('q') | Key::Char('Q') => Action::Quit,
        _ => Action::Nothing,
    }
}

/// Lee una línea de texto del teclado y la devuelve limpia, sin el salto de línea.
fn read_line(prompt: &str) -> String {
    println!("{}", prompt);

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("no he podido leer del teclado");

    line.trim().to_string()
}

/// Pide un número por teclado y repite hasta que el usuario escriba uno válido.
fn read_number(prompt: &str) -> i32 {
    loop {
        match read_line(prompt).parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Eso no es un número. Prueba otra vez."),
        }
    }
}

/// Nombre del mes en español.
fn month_name(month: Month) -> &'static str {
    match month {
        Month::January => "ENERO",
        Month::February => "FEBRERO",
        Month::March => "MARZO",
        Month::April => "ABRIL",
        Month::May => "MAYO",
        Month::June => "JUNIO",
        Month::July => "JULIO",
        Month::August => "AGOSTO",
        Month::September => "SEPTIEMBRE",
        Month::October => "OCTUBRE",
        Month::November => "NOVIEMBRE",
        Month::December => "DICIEMBRE",
    }
}

/// Construye el calendario de un mes como texto: una línea por semana.
/// "today" es una tupla con la fecha de hoy: (año, mes, día).
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
        // Si este día es hoy, lo marcamos con un asterisco en lugar del hueco.
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

fn main() {
    // Term es nuestra ventana al terminal: sirve para leer teclas sueltas.
    let term = Term::stdout();

    // La fecha de hoy, en la zona horaria del equipo.
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
        // Limpiamos la pantalla antes de dibujar: son códigos de control del terminal.
        print!("\x1b[2J\x1b[H");
        print!("{}", month_as_text(year, month, today));
        println!();
        println!("n o -> (flecha derecha) = mes siguiente");
        println!("p o <- (flecha izquierda) = mes anterior");
        println!("q = salir");

        // read_key espera a UNA tecla y la devuelve enseguida.
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

// Las pruebas automáticas viven aquí. No forman parte del programa:
// solo se compilan y se ejecutan con "cargo test".
#[cfg(test)]
mod tests {
    use super::*;

    /// Un "hoy" que no cae en los meses que probamos, para que no salga el asterisco.
    const OTRO_DIA: (i32, Month, u8) = (1999, Month::January, 1);

    #[test]
    fn january_2026_starts_on_thursday() {
        let text = month_as_text(2026, Month::January, OTRO_DIA);
        let first_week = text.lines().nth(2).expect("debería haber una primera semana");

        // Jueves = 3 huecos de 3 caracteres = 9 espacios, más los 2 con los que
        // se alinea el propio número (los números van alineados a la derecha).
        let leading = first_week.len() - first_week.trim_start().len();
        assert_eq!(leading, 11);
        assert_eq!(first_week.trim_start(), "1  2  3  4");
    }

    #[test]
    fn february_2024_has_29_days() {
        let text = month_as_text(2024, Month::February, OTRO_DIA);
        let last_week = text.lines().last().unwrap().trim();
        assert_eq!(last_week, "26 27 28 29");
    }

    #[test]
    fn february_2025_has_28_days() {
        let text = month_as_text(2025, Month::February, OTRO_DIA);
        let last_week = text.lines().last().unwrap().trim();
        assert_eq!(last_week, "24 25 26 27 28");
    }

    #[test]
    fn today_gets_a_star() {
        let text = month_as_text(2026, Month::October, (2026, Month::October, 10));
        assert!(text.contains("9*10"), "esperaba el día 10 marcado:\n{text}");
    }

    #[test]
    fn keys_map_to_actions() {
        assert_eq!(action_for(Key::Char('n')), Action::Next);
        assert_eq!(action_for(Key::Char('P')), Action::Previous);
        assert_eq!(action_for(Key::ArrowRight), Action::Next);
        assert_eq!(action_for(Key::ArrowLeft), Action::Previous);
        assert_eq!(action_for(Key::Char('q')), Action::Quit);
        // Una tecla que no nos interesa no debe hacer nada.
        assert_eq!(action_for(Key::Enter), Action::Nothing);
    }
}
