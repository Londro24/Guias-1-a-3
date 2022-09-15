use std::io::stdin;

fn main() {
    let mut matricula: String = String::new();
    let mut nota_a: String = String::new();
    let mut nota_b: String = String::new();
    let mut nota_c: String = String::new();

    println!("Ingrese debajo la matricula del alumno");
    stdin().read_line(&mut matricula).unwrap();
    println!("Ingrese debajo la primera nota");
    stdin().read_line(&mut nota_a).unwrap();
    println!("Ingrese debajo la segunda nota");
    stdin().read_line(&mut nota_b).unwrap();
    println!("Ingrese debajo la tercera nota");
    stdin().read_line(&mut nota_c).unwrap();
    
    let nota_a_float: f32 = nota_a.trim().parse().unwrap();
    let nota_b_float: f32 = nota_b.trim().parse().unwrap();
    let nota_c_float: f32 = nota_c.trim().parse().unwrap();

    let promedio: f32 = (nota_a_float + nota_b_float + nota_c_float) / 3.0;

    print!("El alumno, de matricula {}, tiene promedio {}", &matricula.trim(), promedio);
}
