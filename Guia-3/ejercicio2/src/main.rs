use std::io::stdin;

fn funcion_1() {
    let mut contador: u8 = 1;
    loop {
        if contador == 11 {
            break;
        }

        let mut numero_texto: String = String::new();
        println!("Ingrese debajo un numero, este es el {}°", contador);
        stdin().read_line(&mut numero_texto).unwrap();
        let numero: i16 = numero_texto.trim().parse().expect("deberia ser un número");

        if numero % 2 == 0 {
            println!("El número {} es par", numero);
        } else {
            println!("El número {} es impar", numero);
        }

        contador += 1;
    }
}

fn funcion_2() {
    let mut lista: String = "".to_string();
    for a in 1..=10000 {
        let mut suma: u32 = 0;
        for b in 1..a {
            if a % b == 0 {
                suma += b
            }
        }
        if a == suma {
            let numero: &str = &a.to_string();
            lista = lista + numero + " ";
        }
    }
    println!("{}", lista);
    println!("por razones de optimización solo se busco hasta el número 10000");
}

fn funcion_3() {
    loop {
        let mut numero_al_revez: String = String::new();
        let mut numero_texto: String = String::new();
        println!("Ingrese debajo un numero");
        stdin().read_line(&mut numero_texto).unwrap();
        if numero_texto.trim().len() == 4 {
            for n in numero_texto.to_string().trim().chars(){
                numero_al_revez = n.to_string() + &numero_al_revez; 
            }
            let numero: u32 = numero_al_revez.trim().parse().unwrap();
            println!("El numero al revez es: {}", numero);
            break;
        } else { 
            println!("vuelva a intentar")
        }
    }
}

fn main() {

    println!("Bienvenido, este es un programa multifuncinonal. Para ello ingrese un número para selecionar la funcion deseada.");
    println!(">>La funcion 1, determina en una seried e 10 números cual es par o impar.");
    println!(">>La funcion 2, llega a devolver hasta 50 números perfectos.");
    println!(">>La funcion 3, invierte el numero ingresado, este debe tener 4 digitos.");
    println!(">>Si ingresa 0 el programa se cerra, con esto en mente uselo tantas veces quiera, solo recuerde volver ingresar el numero de la funcion a fin.");

    loop {
        let mut selector_texto: String = String::new();
        println!("Ingrese debajo el número de la funcion a usar:");
        stdin().read_line(&mut selector_texto).unwrap();
        let selector: u8 = selector_texto.trim().parse().expect("deberia ser un número pequeño");
        
        if selector == 0 {
            println!("Gracias por usar el programa.");
            break;
        } else if selector == 1{
            funcion_1()
        } else if selector == 2 {
            funcion_2()
        } else if selector == 3 {
            funcion_3()
        } else {
            println!("Vuelva a intentar con un numero, ojala valido");
        }
    }
}
