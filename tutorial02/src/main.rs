use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("intenta adivinar el numero");

    let numero = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Ingrese el numero");

        let mut entrada = String::new();

        io::stdin()
            .read_line(&mut entrada)
            .expect("no se pudo leer la linea");

        let entrada: u32 = match entrada.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("ingrese un numero valido");
                continue;
            }
        };
        
        match entrada.cmp(&numero) {
            Ordering::Less => println!("muy bajo!"),
            Ordering::Greater => println!("muy alto!"),
            Ordering::Equal => {
                println!("Ganaste");
                break;
            }
        }

    }
}
