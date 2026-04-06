use std::io;


fn main() {

// Variables y mutabilidad
   let y = 5;
   //  println!("The value of y is: {y}");
   // y = 6;
    println!("The value of y is: {y}");


    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

// Constantes
// Deben llevar siempre declarado el tipo de dato; a diferencia de las variables mutables; las constantes NUNCA podrán cambiar su valor.
// La conveccion para nombrar constantes es USAR MAYUSCULAS_SEPARADAS_POR_GUION_BAJO
    const SUELDO_MINIMO: u32 = 560000;
    println! ("el sueldo minimo actual es {SUELDO_MINIMO}");


// Sombreado o eclipsado de una 
// Sombrear una palabra es distinto a usar el MUT pues, si intentamos cambiar el valor sin el mut, el compilador dará error.
// Lo que mantiene a la variable como inmutable a lo largo del código.

    let c = 5;

    let c = c + 1;

    {
        let c = c * 2;
        println!("The value of c in the inner scope is: {c}");
    }

    println!("The value of c is: {c}");

// También nos permite reutilizar el nombre de variables y simplemente cambiarles el tipo de dato
     let mut spaces = String::new();
     io::stdin()
        .read_line(&mut spaces)
        .expect("Error leyendo");
    
     let spaces = spaces.trim().len();

     println!("Cantidad de caracteres: {}", spaces);

}
