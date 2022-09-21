use std::io::stdin;

fn is_number_valid(n: &str) -> bool {
    if n.chars().next().unwrap().is_numeric() {
        return true;
    }
    return false
}

fn is_panic(contador: u8, dia: u16, mes: u16) -> bool {
    if contador != 5 {
        return true;
    } else if dia > 29 && mes == 2 {
        return true;
    } else if dia > 30 && (mes == 4 || mes == 6 || mes == 9 || mes == 11 ) {
        return true;
    } else if dia > 31 && (mes == 1 || mes == 3 || mes == 5 || mes == 7 || mes == 8 || mes == 10 || mes == 12) {
        return true;
    } else {
        return false;
    }
}

fn pasar_a_texto(mes: u16) -> String {
    let mes_en_texto: String = match mes {
        1_u16 => "enero".to_string(),
        2 => "febrero".to_string(),
        3 => "marzo".to_string(),
        4 => "abril".to_string(),
        5 => "mayo".to_string(),
        6 => "junio".to_string(),
        7 => "julio".to_string(),
        8 => "agosto".to_string(),
        9 => "septiembre".to_string(),
        10 => "octubre".to_string(),
        11 => "noviembre".to_string(),
        12 => "diciembre".to_string(),
        _ => "".to_string()
    };
    return mes_en_texto
}

fn main() {
    let mut dia_texto: String = String::new();
    let mut mes_texto: String = String::new();
    let mut dia_ingresado: String = String::new();
    println!("Ingrese debajo un dia en especifico, el formato es 'XX/XX'");
    stdin().read_line(&mut dia_ingresado).unwrap();
    
    let mut contador: u8 = 1;
    for n in dia_ingresado.to_string().trim().chars(){
        if is_number_valid(&n.to_string()) {
            if contador == 1 {
                mes_texto = mes_texto + &n.to_string();
                contador += 1; 
            } else if contador == 2 {
                mes_texto = mes_texto + &n.to_string();
                contador += 1; 
            } else if contador == 3 {
                dia_texto = dia_texto + &n.to_string();
                contador += 1; 
            } else if contador == 4 {
                dia_texto = dia_texto + &n.to_string();
                contador += 1; 
            } else {
                contador += 1;
            }
            println!("{} de {}", mes_texto, dia_texto);
        }
    }

    let dia: u16 = dia_texto.trim().parse().unwrap();
    let mes: u16 = mes_texto.trim().parse().unwrap();

    if is_panic(contador, dia, mes) {
        panic!("ingrese un dia valido")
    }

    let mes_en_texto: String = pasar_a_texto(mes);

    println!("El día de hoy es {} de {}.", &dia_texto, &mes_en_texto);

}
