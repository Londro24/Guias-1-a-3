use std::io::stdin;

fn numero_mayor(numero_a_ingresar: i32, mayor: i32, menor: i32, contador: u32) -> i32 {
    if contador == 1 {
        let mayor: i32 = numero_a_ingresar;
        return mayor;
    } else {
        if numero_a_ingresar > mayor && numero_a_ingresar > menor{
            let mayor: i32 = numero_a_ingresar;
            return mayor;
        } else {
            return mayor;
        }
    }
}

fn numero_menor(numero_a_ingresar: i32, mayor: i32, menor: i32 , contador: u32) -> i32 {
    if contador == 1 {
        let menor: i32 = numero_a_ingresar;
        return menor;
    } else {
        if numero_a_ingresar < mayor && numero_a_ingresar < menor {
            let menor: i32 = numero_a_ingresar;
            return menor;
        } else {
            return menor;
        }
    }
}


fn main() {

    println!("Bienvenido, este es un programa donde hay un ingreso de números 
    indefinido hasta que ingrese el '0'");
    
    let mut mayor: i32 = 0;
    let mut menor: i32 = 0;
    let mut contador: u32 = 1;

    loop {
        let mut numero: String = String::new();
        println!("Ingrese debajo un número");
        stdin().read_line(&mut numero).unwrap();
        let numero_a_ingresar: i32 = numero.trim().parse().unwrap();

        mayor = numero_mayor(numero_a_ingresar, mayor, menor, contador);
        menor = numero_menor(numero_a_ingresar, mayor, menor, contador);

        println!("El numero mayor es {}, y el menor es {}", mayor, menor);

        if numero_a_ingresar == 0 {
            break;
        }
        contador += 1
    }
}
