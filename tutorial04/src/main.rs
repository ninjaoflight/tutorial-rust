
//notacion de slices
fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
    let mut mi_cadena = String::from("hola mundo");

    // primera_palabra funciona en un slice de una cadena(String)
    let palabra = primera_palabra(&mi_cadena[..]);

    // mi_cadena.clear(); // no se puede debido a la referencia

    println!("{} (String)", palabra);

    mi_cadena.clear(); // aqui si se puede ya que no hay una referencia en uso

    let cadena_literal = "hola mundo";

    // primera_palabra funciona en un slice de una cadena literal(str)
    let palabra = primera_palabra(&cadena_literal[..]);

    println!("{} (str)", palabra);

    // las cadenas literales ya son slices
    // asi que no es nescesario usar la sintaxis slice
    let palabra = primera_palabra(cadena_literal);

    println!("{}", palabra);

}
