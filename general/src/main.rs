use std::{io::stdin, convert::TryInto};

fn is_number_valid(n: &str) -> bool {
    if n.chars().next().unwrap().is_numeric() {
        return true;
    }
    return false
}

fn proceso_magico(rut: &str) -> u32 {
    let mut rut_al_revez: String = String::new();
    let mut contador: u32 = 1; //rut.len().try_into().unwrap();
    let mut rut_numero: u32 = 0;

    for n in rut.to_string().trim().chars(){
        rut_al_revez = n.to_string() + &rut_al_revez; 
    }

    for n in rut_al_revez.to_string().trim().chars(){
        if contador == 1 {
            rut_numero += (n as u32 - '0' as u32) * 2;
        } else if contador == 2 {
            rut_numero += (n as u32 - '0' as u32) * 3;
        } else if contador == 3 {
            rut_numero += (n as u32 - '0' as u32) * 4;
        } else if contador == 4 {
            rut_numero += (n as u32 - '0' as u32) * 5;
        } else if contador == 5 {
            rut_numero += (n as u32 - '0' as u32) * 6;
        } else if contador == 6 {
            rut_numero += (n as u32 - '0' as u32) * 7;
        } else if contador == 7 {
            rut_numero += (n as u32 - '0' as u32) * 2;
        } else if contador == 8 {
            rut_numero += (n as u32 - '0' as u32) * 3;
        }
        contador += 1
    }

    return 0;
}

fn main() {

    println!("Bienvenido, este es un programa para conseguir el digito verificador de un RUT");
    
    let mut rut_a_ingresar: String = String::new();
    println!("Ingrese debajo el rut:");
    stdin().read_line(&mut rut_a_ingresar).unwrap();

    let mut rut_texto: String = String::new();
    for n in rut_a_ingresar.to_string().trim().chars(){
        if is_number_valid(&n.to_string()){
            rut_texto = rut_texto + &n.to_string(); 
        }
    }
    
    let _rut: u32 = rut_texto.trim().parse().expect("ya pero, almenos ingresa un número");

    let digito_verificador: u32 = proceso_magico(&rut_texto);

    let rut_original: &str = &rut_a_ingresar.trim();


    
    println!("El rut {} tiene el siguiente digito verificardor {}", rut_original, digito_verificador);

}
