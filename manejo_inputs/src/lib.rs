use std::io::stdin;
use operaciones;

pub fn mostrar_menu() {
    println!("OPERACIONES DISPONIBLES");
    println!("1. Suma");
    println!("2. Resta");
    println!("3. Multiplicacion");
    println!("4. Division");
}

pub fn obtener_input(label: &str) -> i32 {
    println!("{}", label);

    // Obteniendo el input
    let mut input_string = String::new();
    stdin().read_line(&mut input_string).unwrap();

    // Limpiando el input de los caracteres de retorno
    input_string = input_string.replace("\n", "");
    input_string = input_string.replace("\r", "");

    // Convirtiendo el input a un entero
    let numero: i32 = input_string.parse::<i32>().unwrap();
    numero
}

pub fn obtener_resultado(opcion: i32, a: i32, b: i32) -> i32 {
    match opcion {
        1 => operaciones::suma(a, b),
        2 => operaciones::resta(a, b),
        3 => operaciones::multiplicacion(a, b),
        4 => operaciones::division(a, b),
        _ => 0
    }
}
