use std::io::stdin;

fn funcion(numero: u32) -> u32 {
    let mut suma:u32 = 0;
    for n in 1..=numero {
        for m in 1..=numero {
            if n == m {
                suma += m.pow(n)
            }
        }
    }
    return suma;
}

fn main() {

    println!("Bienvenido, este es un programa para encontrar la suam del 1 a número que ingrese elevado al mismo número en cuestion");

    let mut numero_texto: String = String::new();
    println!("Ingrese debajo el número de la funcion a usar:");
    stdin().read_line(&mut numero_texto).unwrap();
    let numero: u32 = numero_texto.trim().parse().expect("deberia ser un número pequeño");

    let suma: u32 = funcion(numero);
    
    println!("La suma hasta el numero {} es {}", numero, suma)
}
