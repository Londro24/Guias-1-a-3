use std::io::stdin;

fn numero_par(mut numero:u32) -> u32 {
    numero = numero / 2;
    return numero;
}

fn numero_impar(mut numero: u32) -> u32 {
    numero = (numero * 3) + 1;
    return numero;
}


fn main() {

    println!("Bienvenido, este es un programa donde el número que ingrese se le realizará la conjetura de ULAM, comentando tmabien el número de pasos que hizo.");
    
    let mut contador: u32 = 1;

    loop {
        let mut numero_a_ingresar: String = String::new();
        println!("Ingrese debajo un número");
        stdin().read_line(&mut numero_a_ingresar).unwrap();
        let mut numero: u32 = numero_a_ingresar.trim().parse().unwrap();
        
        if numero == 0 {
            println!("Error, el programa volvera a iniciar");
            continue;
        }

        let mut numero_original:u32 = 0;
        numero_original = numero_original + numero;

        while numero != 1 {
            if numero % 2 == 0 {
                numero = numero_par(numero)
            } else {
                numero = numero_impar(numero)
            }
            contador += 1
        }

        println!("Al numero {} tomo {} pasos para llegar al número 1", numero_original, contador);

        if numero == 1 {
            break;
        }
    }

}
