use std::{io::stdin, convert::TryInto};

fn is_a_mayor_b(palabra_a: &str, palabra_b: &str) -> bool {
    let cuenta_a:u32 = palabra_a.len().try_into().unwrap();
    let cuenta_b:u32 = palabra_b.len().try_into().unwrap();
    cuenta_a > cuenta_b
}

fn diferencia_a_or_b_mayor(palabra_a: &str, palabra_b: &str) -> u32{
    let cuenta_a:u32 = palabra_a.len().try_into().unwrap();
    let cuenta_b:u32 = palabra_b.len().try_into().unwrap();
    if cuenta_a > cuenta_b {
        return cuenta_a - cuenta_b;
    } else {
        return cuenta_b - cuenta_a;
    }
}

fn main() {

    let mut palabra_a: String = String::new();
    let mut palabra_b: String = String::new();

    println!("Ingrese debajo la 1era palabra");
    stdin().read_line(&mut palabra_a).unwrap();
    println!("Ingrese debajo la 2da palabra");
    stdin().read_line(&mut palabra_b).unwrap();

    let palabra_a: &str = &palabra_a.trim();
    let palabra_b: &str = &palabra_b.trim();

    if is_a_mayor_b(palabra_a, palabra_b){
        let diferencia:u32 = diferencia_a_or_b_mayor(palabra_a, palabra_b);
        println!("La palabra '{}' es mayor por {} letras a la palabra '{}'",
        palabra_a, diferencia, palabra_b)
    } else {
        let diferencia:u32 = diferencia_a_or_b_mayor(palabra_a, palabra_b);
        println!("La palabra '{}' es mayor por {} letras a la palabra '{}'",
        palabra_b, diferencia, palabra_a)
    }
}
