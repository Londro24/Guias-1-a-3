use std::io::stdin;

fn main() {
    let mut cateto_a: String = String::new();
    let mut cateto_b: String = String::new();

    println!("Ingrese el primer cateto");
    stdin().read_line(&mut cateto_a).unwrap();
    println!("Ingrese el segundo");
    stdin().read_line(&mut cateto_b).unwrap();
    
    let cateto_a_float: f32 = cateto_a.trim().parse().unwrap();
    let cateto_b_float: f32 = cateto_b.trim().parse().unwrap();

    let hipotenusa: f32 = (cateto_a_float.powf(2.0) + cateto_b_float.powf(2.0)).powf(0.5);

    print!("la hipotenusa es {}", hipotenusa);
}
