use std::io::stdin;

fn main() {
    let mut altura: String = String::new();
    let mut base: String = String::new();

    println!("Ingrese la altura");
    stdin().read_line(&mut altura).unwrap();
    println!("Ingrese la base");
    stdin().read_line(&mut base).unwrap();
    
    let altura_float: f32 = altura.trim().parse().unwrap();
    let base_float: f32 = base.trim().parse().unwrap();

    let superficie: f32 = base_float * altura_float;
    let perimetro: f32 = 2.0 * (base_float + altura_float);
    print!("Su perimetro es de {}u y superficie es de {}u cuadradas", perimetro, superficie);
}
