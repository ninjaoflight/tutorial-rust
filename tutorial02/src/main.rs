use std::io;

fn main() {
    println!("intenta adivinar el numero");
    println!("Ingrese el numero");

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("no se pudo leer la linea");

    println!("el numero es {}", entrada);
}