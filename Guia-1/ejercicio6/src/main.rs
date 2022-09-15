use std::io::stdin;

fn main() {
    let mut galones: String = String::new();

    println!("Ingrese debajo los galones en venta");
    stdin().read_line(&mut galones).unwrap();
    
    let galones_float: f32 = galones.trim().parse().unwrap();

    let litro: f32 = galones_float * 3.785;
    let precio: f32 = litro * 820.0;

    println!("Se vendieron {} litros al precio de ${} pesos", 
    litro, precio);
}
