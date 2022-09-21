use std::io::stdin;

fn evaluar(x: f32) -> f32 {
    let x_final: f32 = x.powf(3.0) + 2.0 * x.powf(2.0) + 3.0 *  x + 5.0;
    return x_final;
}

fn main() {
    let mut x_ingresado: String = String::new();

    println!("Ingrese debajo un numero a validar");
    stdin().read_line(&mut x_ingresado).unwrap();
    
    let x: f32 = x_ingresado.trim().parse().expect("Se esperaba almenos un número");

    let x_final: f32 = evaluar(x);

    println!("Su numero es {}", x_final);
}
