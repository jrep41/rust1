// Versión 5: marcamos el día de hoy y usamos una sola librería de fechas: time.
// (chrono necesitaba para esto herramientas de MinGW que en este equipo faltan.)

use std::io;
use time::{Date, Month, OffsetDateTime};

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

// Dibuja un mes. "today" es una tupla con la fecha de hoy: (año, mes, día).
fn print_month(year: i32, month: Month, today: (i32, Month, u8)) {
    // Cuántos días tiene el mes. time ya sabe los años bisiestos.
    let days_in_month = month.length(year);

    // El día 1 de ese mes.
    let first_day = Date::from_calendar_date(year, month, 1).expect("fecha no válida");

    // Huecos vacíos antes del día 1: lunes = 0, martes = 1, ... domingo = 6.
    let blank_cells = first_day.weekday().number_days_from_monday();

    let title = format!("{} {}", month_name(month), year);
    println!("{:^21}", title);
    println!(" Lu Ma Mi Ju Vi Sá Do");

    let mut column = 0;
    for _ in 0..blank_cells {
        print!("   ");
        column += 1;
    }

    for day in 1..=days_in_month {
        // Si este día es hoy, lo marcamos con un asterisco en lugar del hueco.
        if year == today.0 && month == today.1 && day == today.2 {
            print!("*{:>2}", day);
        } else {
            print!("{:>3}", day);
        }

        column += 1;
        if column == 7 {
            println!();
            column = 0;
        }
    }

    if column != 0 {
        println!();
    }
}

fn main() {
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
        print_month(year, month, today);
        println!();
        println!("n = mes siguiente   p = mes anterior   q = salir");

        let command = read_line("> ");
        match command.as_str() {
            "n" => {
                month = month.next();
                if month == Month::January {
                    year += 1;
                }
            }
            "p" => {
                month = month.previous();
                if month == Month::December {
                    year -= 1;
                }
            }
            "q" => break,
            _ => println!("No te he entendido. Usa n, p o q."),
        }
    }

    println!("¡Hasta luego!");
}
