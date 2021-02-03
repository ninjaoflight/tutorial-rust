// 1. farenheit -> celcius

fn to_celcius(farenheit: f32) -> f32 {
    (farenheit - 32.0) / 1.8
}

// 2. fibonacci
fn fibbonacci(num: i32) -> Result<i32, String> {
    if num < 0 {
        return Err(format!("{} es un numero negativo!", num));
    }

    match num {
        0 | 1 => Ok(1),
        _ => {
            let mut c: (i32, i32, i32) = (1, 1, 0);
            for _ in 1..num {
                c.2 = c.1;
                c.1 = c.0;
                c.0 = c.1 + c.2;
            }
            Ok(c.0)
        }
    }
}
// 3. 12 días navidad
fn cancion() {
    let dias = [
        ("primer", "una perdiz en un peral"),
        ("segundo", "dos tórtolas"),
        ("tercero", "tres gallinas francesas"),
        ("cuarto", "cuatro pájaros cantores"),
        ("quinto", "cinco anillos de oro"),
        ("sexto", "seis gansos sentados"),
        ("septimo", "siete sisnes nadando"),
        ("octavo", "ocho sirvientas ordeñando"),
        ("noveno", "nueve damas bailando"),
        ("decimo", "diez señores saltando"),
        ("onceavo", "once gaiteros que instalan tubos"),
        ("doceavo", "doce bateristas tocando"),
    ];

    for dia in 0..12 {
        println!("en el {} día de navidad mi verdadero amor me envió {}", dias[dia].0, dias[dia].1);
        for anterior in (0..dia).rev(){
            println!(" -> {}", dias[anterior].1);
        }
    }
}

fn main() {
    let farenheit: f32 = 4.0;
    println!("{}ºF = {}ºC", farenheit, to_celcius(farenheit));
    let num = 5;
    for n in 1..num {
        println!("fibonacci({}) = {}", n, fibbonacci(n).unwrap());
    }
    cancion();
}
