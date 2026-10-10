// Versión 4: ya podemos movernos de mes en mes sin salir del programa.

use chrono::{Datelike, Months, NaiveDate};
use std::io;

/// Lee una línea de texto del teclado y la devuelve limpia, sin el salto de línea.
fn read_line(prompt: &str) -> String {
    println!("{}", prompt);

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("no he podido leer del teclado");

    // trim() quita espacios y el salto de línea; to_string() lo copia a un String nuevo.
    line.trim().to_string()
}

/// Pide un número por teclado y repite hasta que el usuario escriba uno válido.
fn read_number(prompt: &str) -> i32 {
    loop {
        // Una función puede apoyarse en otra: aquí reutilizamos read_line.
        match read_line(prompt).parse::<i32>() {
            Ok(number) => return number,
            Err(_) => println!("Eso no es un número. Prueba otra vez."),
        }
    }
}

// Una función: un trozo de programa con nombre que podemos usar las veces que queramos.
fn print_month(year: i32, month: u32) {
    // El día 1 de ese mes. "opt" significa que devuelve Option:
    // Some(fecha) si la fecha existe, None si no (por ejemplo, mes 13).
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("fecha no válida");

    // El día 1 del mes siguiente: la distancia entre ambos es el número de días del mes.
    let next_month = first_day
        .checked_add_months(Months::new(1))
        .expect("mes fuera de rango");
    let days_in_month = (next_month - first_day).num_days();

    // Nombre del mes en español. "match" compara el valor y elige una rama.
    let month_name = match month {
        1 => "ENERO",
        2 => "FEBRERO",
        3 => "MARZO",
        4 => "ABRIL",
        5 => "MAYO",
        6 => "JUNIO",
        7 => "JULIO",
        8 => "AGOSTO",
        9 => "SEPTIEMBRE",
        10 => "OCTUBRE",
        11 => "NOVIEMBRE",
        12 => "DICIEMBRE",
        _ => "DESCONOCIDO",
    };

    // Huecos vacíos antes del día 1: 0 = lunes, 1 = martes, ... 6 = domingo.
    let blank_cells = first_day.weekday().num_days_from_monday();

    // Título centrado y fila con los nombres de los días.
    let title = format!("{} {}", month_name, year);
    println!("{:^21}", title);
    println!(" Lu Ma Mi Ju Vi Sá Do");

    // "column" lleva la cuenta de en qué columna de la semana vamos (0 a 6).
    let mut column = 0;

    // Huecos vacíos antes del día 1.
    for _ in 0..blank_cells {
        print!("   ");
        column += 1;
    }

    // Pintamos los días, uno detrás de otro.
    for day in 1..=days_in_month {
        print!("{:>3}", day);
        column += 1;

        // Al llegar al domingo (7 columnas) bajamos de línea.
        if column == 7 {
            println!();
            column = 0;
        }
    }

    // Si el mes no termina en domingo, cerramos la última línea.
    if column != 0 {
        println!();
    }
}

fn main() {
    println!("Calendario: escribe el año y el mes que quieres ver.");
    let mut year = read_number("Año: ");
    let mut month = read_number("Mes (1 a 12): ");

    if month < 1 || month > 12 {
        println!("El mes tiene que estar entre 1 y 12.");
        return;
    }

    // Bucle principal: pintamos el mes y preguntamos qué hacer a continuación.
    loop {
        print_month(year, month as u32);
        println!();
        println!("n = mes siguiente   p = mes anterior   q = salir");

        let command = read_line("> ");

        // match compara el texto escrito con cada opción; "_" recoge lo que no encaje.
        match command.as_str() {
            "n" => {
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
            }
            "p" => {
                month -= 1;
                if month < 1 {
                    month = 12;
                    year -= 1;
                }
            }
            "q" => break,
            _ => println!("No te he entendido. Usa n, p o q."),
        }
    }

    println!("¡Hasta luego!");
}
