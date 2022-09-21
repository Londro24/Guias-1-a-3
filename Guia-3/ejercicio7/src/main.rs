use std::io::stdin;

fn evaluar(numero: u32) -> String {
    let texto: String = match numero {
        1 => "uno".to_string(),
        2 => "dos".to_string(),
        3 => "tres".to_string(),
        4 => "cuatro".to_string(),
        5 => "cinco".to_string(),
        6 => "seis".to_string(),
        7 => "siete".to_string(),
        8 => "ocho".to_string(),
        9 => "nueve".to_string(),
        10 => "diez".to_string(),
        11 => "once".to_string(),
        12 => "doce".to_string(),
        13 => "trece".to_string(),
        14 => "catroce".to_string(),
        15 => "quince".to_string(),
        16 => "dieciseis".to_string(),
        17 => "diecisiete".to_string(),
        18 => "dieciocho".to_string(),
        19 => "diecinueve".to_string(),
        20 => "veinte".to_string(),
        21 => "veintiuno".to_string(),
        22 => "veintidos".to_string(),
        23 => "veintitres".to_string(),
        24 => "veinticuatro".to_string(),
        25 => "veinticinco".to_string(),
        26 => "veintiseis".to_string(),
        27 => "veintisiete".to_string(),
        28 => "veintiocho".to_string(),
        29 => "veintinueve".to_string(),
        30 => "treinta".to_string(),
        40 => "cuarenta".to_string(),
        50 => "cincuenta".to_string(),
        60 => "sesenta".to_string(),
        70 => "setenta".to_string(),
        80 => "ochenta".to_string(),
        90 => "noventa".to_string(),
        _ => "".to_string()
    };
    return texto;
}

fn main() {
    let mut num_ingresado: String = String::new();
    println!("Ingrese debajo un numero entre el 1 y el 99");
    stdin().read_line(&mut num_ingresado).unwrap();
    let numero: u32 = num_ingresado.trim().parse().expect("deberia ser un número");
    if numero < 1 || numero > 99 {
        panic!("Debe ser un número entre 1 y 99")
    }
    
    if numero < 31 || numero % 10 == 0 {
        let numero_texto: String = evaluar(numero);
        println!("El número {} escrito es '{}'.", numero, numero_texto);
    } else {
        let decenas: u32 = numero / 10 * 10;
        let unidades: u32 = numero % 10;
        let numero_texto: String = evaluar(decenas) + &" y ".to_string() + &evaluar(unidades).to_string();
        println!("El número {} escrito es '{}'.", numero, numero_texto);
    }
}
