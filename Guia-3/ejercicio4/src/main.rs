use std::io::stdin;

fn is_number_valid(n: &str) -> bool {
    if n.chars().next().unwrap().is_numeric() {
        return true;
    }
    return false
}

fn mas_segundo(hora: String) -> String {
    let mut new_segundo: String = String::new();
    let mut contador: u8 = 1;
    let mut segundo: u32 = 0;

    for n in hora.chars() {
        if contador == 5 {
            segundo += (n as u32 - '0' as u32) * 10
        } else if contador == 6 {
            segundo += n as u32 - '0' as u32
        }

        contador += 1;
    }

    segundo += 1;
    contador = 1;

    if segundo != 60 {
        let mut new_minuto: u32 = 0;

        for n in hora.chars() {
            if contador == 3 {
                new_minuto += (n as u32 - '0' as u32) * 10
            } else if contador == 4 {
                new_minuto += n as u32 - '0' as u32
            }

            contador += 1;
        }

        new_minuto += 1;
        contador = 1;

        if new_minuto != 60 {
            for n in hora.chars() {
                if contador == 5 && segundo > 10{
                    new_segundo = new_segundo + ":" + &segundo.to_string()
                } else if contador == 5 && segundo < 10 {
                    new_segundo = new_segundo + ":" + &"0".to_string()
                } else if contador == 6 && segundo < 10 {
                    new_segundo = new_segundo + &segundo.to_string()
                } else if contador == 3 {
                    new_segundo = new_segundo + ":" + &new_minuto.to_string()
                } else if contador != 4 && contador != 6{
                    new_segundo = new_segundo + &n.to_string()
                }
                
                contador += 1
            }
        } else {
            if new_minuto == 60 {
                new_minuto = 00;
            }
            let mut new_hora: u32 = 0;

            for n in hora.chars() {
                if contador == 1 {
                    new_hora += (n as u32 - '0' as u32) * 10
                } else if contador == 2 {
                    new_hora += n as u32 - '0' as u32
                }
    
                contador += 1;
            }

            new_hora += 1;
            if new_hora == 24 {
                new_hora = 00;
            }
            contador = 1;

            for _n in hora.chars() {
                if contador == 5 && segundo > 10{
                    new_segundo = new_segundo + ":" + &segundo.to_string()
                } else if contador == 5 && segundo < 10 {
                    new_segundo = new_segundo + ":" + &"0".to_string()
                } else if contador == 6 {
                    new_segundo = new_segundo + &segundo.to_string()
                } else if contador == 3 {
                    new_segundo = new_segundo + ":" + &new_minuto.to_string()
                } else if contador == 4 {
                    new_segundo = new_segundo + &new_minuto.to_string()
                } else if contador == 1 {
                    new_segundo = new_segundo + &new_hora.to_string()
                } else if contador == 2 && new_hora == 0 {
                    new_segundo = new_segundo + &new_hora.to_string()
                }
                
                contador += 1
            }
        }
    } else {
        let mut new_minuto: u32 = 0;
        segundo = 00;

        for n in hora.chars() {
            if contador == 3 {
                new_minuto += (n as u32 - '0' as u32) * 10
            } else if contador == 4 {
                new_minuto += n as u32 - '0' as u32
            }

            contador += 1;
        }

        new_minuto += 2;
        contador = 1;

        if new_minuto < 60 {
            for n in hora.chars() {
                if contador == 5 {
                    new_segundo = new_segundo + ":" + &segundo.to_string()
                } else if contador == 6 {
                    new_segundo = new_segundo + &segundo.to_string()
                } else if contador == 3 {
                    new_segundo = new_segundo + ":" + &new_minuto.to_string()
                } else if contador != 4 {
                    new_segundo = new_segundo + &n.to_string()
                }
                
                contador += 1
            }
        } else {
            if new_minuto == 60 {
                new_minuto = 00;
            } else if new_minuto == 61 {
                new_minuto = 1;
            }
            let mut new_hora: u32 = 0;

            for n in hora.chars() {
                if contador == 1 {
                    new_hora += (n as u32 - '0' as u32) * 10
                } else if contador == 2 {
                    new_hora += n as u32 - '0' as u32
                }
    
                contador += 1;
            }

            new_hora += 1;
            if new_hora == 24 {
                new_hora = 00;
            }
            contador = 1;
            
            for _n in hora.chars() {
                if contador == 5 {
                    new_segundo = new_segundo + ":" + &segundo.to_string()
                } else if contador == 6 {
                    new_segundo = new_segundo + &segundo.to_string()
                } else if contador == 3 && new_minuto == 0 {
                    new_segundo = new_segundo + ":" + &new_minuto.to_string()
                } else if contador == 3 && new_minuto == 1 {
                    new_segundo = new_segundo + ":" + &"0".to_string()
                } else if contador == 4 {
                    new_segundo = new_segundo + &new_minuto.to_string()
                } else if contador == 1 {
                    new_segundo = new_segundo + &new_hora.to_string()
                } else if contador == 2 && new_hora == 0 {
                    new_segundo = new_segundo + &new_hora.to_string()
                }
                
                contador += 1
            }
        }
    }

    return new_segundo;
}

fn main() {

    println!("Bienvenido, este es un programa para saber la hora tras un minuto y segundo despues");

    let mut hora: String = String::new();
    let mut _hora_2: String = String::new();
    let mut hora_texto: String = String::new();
    println!("Ingrese debajo la hora:");
    stdin().read_line(&mut hora_texto).unwrap();

    for n in hora_texto.to_string().trim().chars(){
        if is_number_valid(&n.to_string()) {
            hora = hora + &n.to_string(); 
        }
    }

    if hora.len() != 6 {
        panic!("El formato es erroneo")
    }

    _hora_2 = _hora_2 + &hora;
    let new_hora_2: String = mas_segundo(_hora_2);

    println!("la nueva hora seria: {}", &new_hora_2);
}
