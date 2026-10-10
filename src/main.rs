// Versión 2: los datos del mes ya no están escritos a mano.
// Usamos la librería chrono para saber cuántos días tiene y en qué día empieza.

use chrono::{Datelike, Months, NaiveDate};

// Una función: un trozo de programa con nombre que podemos usar las veces que queramos.
// Entre paréntesis van los datos que necesita (parámetros): el año y el mes.
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
    // Aquí decidimos qué mes queremos ver. Cambia estos dos números y prueba.
    print_month(2026, 1);
}
