use std::io::stdin;

fn main() {
    let mut kilometros: String = String::new();

    println!("Ingrese debajo la distancia en kilometros");
    stdin().read_line(&mut kilometros).unwrap();
    
    let monto_base: f32 = 3000.0;
    let kilometros_float: f32 = kilometros.trim().parse().unwrap();

    let precio: f32 = monto_base + 20.0 * kilometros_float;

    println!("El precio del pasaje es ${}", precio);
}
