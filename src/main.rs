use std::io;

fn leer_numero(mensaje: &str) -> i32 {
    loop {
        println!("{mensaje}");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la línea");

        match entrada.trim().parse::<i32>() {
            Ok(num) if num > 0 && num < 100 => return num,
            _ => println!("Número inválido. Debe ser positivo y menor que 100."),
        }
    }
}

fn main() {
    println!("Usando Rust!");

    // Nombre y suma basica.
    /* 
    println!("Por favor, introduce tu nombre:");

    let mut nombre = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la línea");

    let nombre: &str = nombre.trim();
    println!("¡Hola, {nombre}!");
    println!("Sumando dos números...");

    let num1: i32 = leer_numero("Ingresa el primer número:");
    let num2: i32 = leer_numero("Ingresa el segundo número:");

    println!("La suma de {num1} y {num2} es: {}", num1 + num2);
    println!("¡Gracias por usar el programa, {nombre}!");
    */

    /* Reto: 
        Un programa que le pida al usuario un número 
        y cuente desde 1 hasta ese número, diciendo si cada uno es 
        múltiplo de 3 o no.
    */
    
    let numero: i32 = leer_numero("Por favor, introduce un número:");

    for i in 1..=numero {
        if i % 3 == 0 {
            println!("{i} es múltiplo de 3");
        } else {
            println!("{i} no es múltiplo de 3");
        }
    }
}