use std::io::stdin;

fn main() {
    println!("Ingrese un numero");
    loop {
        let mut num: String = String::new();
        stdin().read_line(&mut num).unwrap();
        let num: u32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

    let cuadrado: u32 = num.pow(2);
    let cubo: u32 = num.pow(3);

    print!("El cuadrafo del numero es {}. Y si cubo es {}.", cuadrado, cubo);
    break;
    }
}
