use std::io::stdin;

fn is_palabra(frase: &str, palabra: &str) -> bool {
    let mayuscula: String = palabra.to_uppercase();
    let minuscula: String = palabra.to_lowercase();
    if frase.contains(&palabra) || frase.contains(&mayuscula) || frase.contains(&minuscula) {
        return true
    } else {
        return false
    }
}

fn main() {

    let mut frase: String = String::new();
    let mut palabra: String = String::new();

    println!("Ingrese debajo una frase");
    stdin().read_line(&mut frase).unwrap();
    println!("Ingrese debajo una palabra a encontrar");
    stdin().read_line(&mut palabra).unwrap();

    let frase: &str = frase.trim();
    let palabra: &str = palabra.trim();

    if is_palabra(&frase, &palabra) {
        println!("La palabra '{}' ha sido encontrada", palabra);
    } else {
        println!("La palabra '{}' no ha sido encontrada", palabra);
    }

}
