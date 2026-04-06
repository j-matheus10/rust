use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
// saludo inicial

    println!("Bienvenido al azar!\n");
    println!("Piensa en un número del 1 al 100\n");

loop {
//generación del número al azar por parte de rand

    let numero_azar = rand::thread_rng()
                            .gen_range(1..=100);

// Sistema para input de usuario

    println!("¿Qué numero crees que será?\n");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect ("Error en lectura");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

//Comparación de creados
    match guess.cmp(&numero_azar){
        Ordering::Less => println!("Muy chiquito"),
        Ordering::Greater => println!("Muy grande"),
        Ordering::Equal => {
            println!("Elena");
            break;}
            }
        }
}
