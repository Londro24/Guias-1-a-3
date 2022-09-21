use std::io::stdin;

fn paga(base: f32, comision_int: f32) -> f32 {
    let paga: f32  = base + (7.0 * comision_int) * base / 100.0;
    return paga;
}

fn main() {
    let mut monto_base: String = String::new();
    let mut comisiones: String = String::new();

    println!("Ingrese debajo su sueldo base");
    stdin().read_line(&mut monto_base).unwrap();
    println!("Ingrese debajo la cantidad de comisiones hechas");
    stdin().read_line(&mut comisiones).unwrap();
    
    let base: f32 = monto_base.trim().parse().expect("Se esperaba un número en el monto base");
    let comision_int: f32 = comisiones.trim().parse().expect("Se esperaba un número en las comisinones");

    let paga: f32 = paga(base, comision_int);

    println!("Su sueldo es de ${}", paga);
}
