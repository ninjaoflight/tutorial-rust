// enum clasicos
// si quisieramos almacenar la direccion
// necesitariamos un struct
enum _TipoDireccionIp {
    V4,
    V6,
}

// podemos evitar esto almacenando datos
// en cada variante del enum
enum DireccionIP {
    V4(u8, u8, u8, u8),
    V6(String),
}

// este enum contiene distintos
// tipos embedidos en sus variantes
enum Mensaje{
    Salir,
    Mover {x: i32, y: i32},
    Enviar(String),
    CambiarColor(i32, i32, i32),
}

// tambien podemos definir metodos en los enums
impl Mensaje{
    fn llamar(&self){
        // comportamiento definido aqui
    }
}

fn main() {
    let _localhost = DireccionIP::V4(127, 0, 0, 1);
    let _loopback = DireccionIP::V6(String::from("::1"));

    let m = Mensaje::Enviar(String::from("hola"));
    m.llamar();

    // tambien son usados en Option<T>
    // como caracteristica de rust

    let _algun_numero = Some(5);
    let _alguna_cadena = Some("una cadena");

    // rust no puede deducir el tipo 
    // cuando el valor asignado es None
    let numero_ausente: Option<i32> = None;

    // podemos usar la expresion match
    // para ejecutar una ruta que coincida
    let valido = match numero_ausente{
        Some(valor) =>{
            // podemos introducir codigo
            println!("[{}]", valor);
            String::from("Valido")
        },
        // tambien se puede usar 
        // para el resto de valores
        // _ => String::from("otro")
        None => String::from("Invalido")
    };
    // tambien se puede utilizar la sintaxis 
    // if let, si solo nos interesan pocos casos
    
    if let Some(valor) = numero_ausente {
        if valor > 0{
            println!("positivo");
        }
        else{
            println!("negativo");
        }
    }
    // else{
    //     println!("N/A");
    // }


    println!("el numero es: {}", valido);

}
