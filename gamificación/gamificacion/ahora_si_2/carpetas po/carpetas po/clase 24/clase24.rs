#[derive(Debug)]
struct Brigadista{
    nombre: String,
    edad: u8,
    puntos: i32,
    liga: String,
}
struct Monitoreo{
    usuarios: Vec<Brigadista>,
}

impl Monitoreo{
    fn new() -> Self{
        Monitoreo{
            usuarios: Vec::new(),
        }
    }
    fn crea_usuario(&mut self, brigadista:Brigadista){
        self.usuarios.push(brigadista);
    }
    fn tamano(&self) -> usize {
        self.usuarios.len()
    }
}
impl Brigadista{
    fn alerta(&mut self){
        println!("Soy el brigadista {}", self.nombre);
        self.puntos += 10;
    }
}

fn main(){
    //Creamos una estructura Monitoreo con la función new()
    let mut app_monitoreo = Monitoreo::new();
    let mut b1 = Brigadista{
        nombre: String::from("Juan"),
        edad: 25,
        puntos: 0,
        liga: String::from("Bronce"),
    };
    let mut b2 = Brigadista{
        nombre: String::from("Pedro"),
        edad: 25,
        puntos: 0,
        liga: String::from("Bronce"),
    };
    b1.puntos += 10;
    println!("El primer brigadista es {:?}", b1);
    app_monitoreo.crea_usuario(b1);
    app_monitoreo.crea_usuario(b2);
    println!("La cantidad de usuarios es {}", app_monitoreo.tamano());

}