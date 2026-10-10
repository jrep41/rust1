// Versión 1: mostramos un mes fijo en pantalla.
// Todavía no pedimos datos al usuario ni calculamos fechas.

fn main() {
    // Los datos del mes que vamos a pintar (de momento, escritos a mano).
    let month_name = "ENERO";
    let year = 2026;
    let days_in_month = 31;
    // 0 = lunes, 1 = martes, ... 6 = domingo.
    // El 1 de enero de 2026 cae en jueves, así que sobran 3 huecos al principio.
    let blank_cells = 3;

    // Título: {:^21} centra el texto en 21 columnas (7 días x 3 caracteres).
    let title = format!("{} {}", month_name, year);
    println!("{:^21}", title);

    // Fila con los nombres de los días, cada uno ocupando 3 columnas.
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
