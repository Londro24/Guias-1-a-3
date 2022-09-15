use std::io::stdin;

fn main() {
    let mut dias: String = String::new();

    println!("Ingrese debajo un numero de días");
    stdin().read_line(&mut dias).unwrap();
    
    let dias_int: u32 = dias.trim().parse().unwrap();

    let segundos: u32 = dias_int * 24 * 3600;

    println!("En {} días hay {} segundos", 
    dias_int, segundos);
}
