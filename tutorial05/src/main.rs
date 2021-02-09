struct Usuario {
    nombre_usuario: String,
    correo: String,
    conteo_inicio: u64,
    activo: bool,
}

fn crear_usuario(correo: String, usuario: String) -> Usuario {
    // cuando los nombres coinciden no es necesario
    // indicarlos en el inicializador
    Usuario {
        nombre_usuario: usuario,
        correo,
        conteo_inicio: 1,
        activo: true,
    }
}

// permite usar 
// println!("rectangulo [{:?}]", rectangulo);
// aunque tambien puede usarse sin este
// println!("rectangulo [{:#?}]", rectangulo);
#[derive(Debug)]
struct Rectangulo{
    largo: u32,
    ancho: u32
}

impl Rectangulo{
    fn area(&self) -> u32{
        self.ancho * self.largo
    }
    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.largo > otro.largo
    }
    fn cuadrado(longitud: u32) -> Rectangulo{
        Rectangulo{
            largo: longitud,
            ancho: longitud
        }
    }
}

fn main() {
    let usuario1 = crear_usuario(String::from("jon@doe.net"), String::from("jhon doe"));
    // sintaxis de actualizacion
    // los campos restantes se copian de usuario 1
    let _usuario2 = Usuario {
        nombre_usuario: String::from("Jhon Doe"),
        correo: String::from("jane@doe.net"),
        ..usuario1
    };

    // estructuras de tuplas
    struct Punto(u32, u32, u32);
    struct Color(u32, u32, u32);

    let _punto1 = Punto(0, 0, 0);
    let _negro = Color(0, 0, 0);
    // _punto1 = _negro; error

    let r1 = Rectangulo{
        largo: 8,
        ancho: 6
    };
    let r2 = Rectangulo::cuadrado(4);

    // a diferiencia de C/C++ este referencia
    // y derreferencia automaticamente
    // no hace falta hacer objeto->funcion()
    println!("rectangulo: [{}]", r1.area());
    println!("r1 puede contener a r2: {}", r1.puede_contener(&r2));
    println!("r2 puede contener a r1: {}", r2.puede_contener(&r1));
}
