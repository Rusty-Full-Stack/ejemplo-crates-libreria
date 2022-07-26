use manejo_inputs;

// Funcion Main
fn main() {

    // Mostrando el menu de operaciones
    manejo_inputs::mostrar_menu();

    // Leyendo la operacion que el usuario quiere ejecutar
    let opcion: i32 = manejo_inputs::obtener_input("Escribe el numero de la operacion que quieres realizar");

    // Leyendo los digitos a utilizar.
    let a: i32 = manejo_inputs::obtener_input("Primer digito");
    let b: i32 = manejo_inputs::obtener_input("Segundo digito");

    // Calculando el resultado, por facilidad, si la operacion elegida no es ninguna de las
    // del menu, entonces vamos a colocar un 0
    let resultado = manejo_inputs::obtener_resultado(opcion, a, b);

    // imprimiendo el resultado en pantalla
    println!("Resultado: {}", resultado);
}
