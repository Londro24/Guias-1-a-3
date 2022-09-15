use std::io::stdin;

fn main() {
    let mut dinosaurio: String = String::new();
    let mut libra: String = String::new();
    let mut pie: String = String::new();
    
    println!("Ingrese debajo el nombre del dinosaurio");
    stdin().read_line(&mut dinosaurio).unwrap();
    println!("Ingrese debajo su peso en libras");
    stdin().read_line(&mut libra).unwrap();
    println!("Ingrese debajo su longitud en pies");
    stdin().read_line(&mut pie).unwrap();
    
    let libra_float: f32 = libra.trim().parse().unwrap();
    let pie_float: f32 = pie.trim().parse().unwrap();

    let kilo: f32 = libra_float * 0.45;
    let metro: f32 = pie_float * 0.3048;

    print!("El dinosaurio {}, tiene tiene un peso de {} kilos y tiene una longitud de {} metros", 
    &dinosaurio.trim(), kilo, metro);
}
