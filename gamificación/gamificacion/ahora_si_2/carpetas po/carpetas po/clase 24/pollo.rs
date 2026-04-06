#[derive(Debug)]
struct Usuario {
    nombre : String,
    edad : u8,
    puntos : i16,
}


fn main(){

let pepe = Usuario {
    nombre: String::from("Pepe"),
    edad:25,
    puntos:0,

};
println!("Nombre {:?}", pepe)
}

