#[derive(Debug)]
struct Brigadista {
    nombre: String,
    edad: u8,
    puntos: i32,
    liga: String,
}
struct liga {
    nombre: String,
    inferior: i32,
    superior: i32,
    ranking: Vec<Brigadista>,
    ponderador: f32,
}

struct Monitoreo{
    Ligas: Vec<liga>,
}
impl liga {

    fn new(n:String) -> Self {
        liga {
            nombre: n,
            superior: 0,
            inferior: 0,
            ponderador: 0.0,
            ranking: Vec::new(),
        }
    }

    fn crea_usuario(&mut self, brigadista:Brigadista) {
        self.ranking.push(brigadista);
    }

    fn tamano(&self) -> usize {
        self.nombre.len()
    }
}
impl Monitoreo{
    fn new() -> Self {
        Monitoreo {
            Ligas: Vec::new(),
        }
    }

    fn crea_liga(&mut self, Liga:liga) {
        self.Ligas.push(Liga);
    }

    fn tamano(&self) -> usize {
        self.Ligas.len()
    }
}

impl Brigadista {
    fn alerta(&mut self){
        println!("Soy el brigadista {}", self.nombre);
        self.puntos += 10;
    }
}

fn main(){
    // Creamos una estructura Monitoreo con la función new()
    let mut app_monitoreo = Monitoreo::new();
    let mut bronze = liga::new(String::from("Bronce"));
    let mut b1 = Brigadista {
        nombre: String::from("Juan"),
        edad: 25,
        puntos: 0,
        liga: String::from("Bronce"),
    };
    let mut b2 = Brigadista {
        nombre: String::from("Angélica"),
        edad: 29,
        puntos: 0,
        liga: String::from("Bronce"),
    };
    println!("El primer brigadista es {:?}", b1);
    println!("El segundo brigadista es {:?}", b2);
    b1.puntos += 10;
    b1.alerta();
    println!("El primer brigadista es {:?}", b1);
    bronze.crea_usuario(b1);
    println!("La cantidad de usuarios es {}", bronze.tamano());
    bronze.crea_usuario(b2);
    println!("La cantidad de usuarios es {}", bronze.tamano());
    app_monitoreo.crea_liga(bronze);
    println!("La cantidad de ligas es {}", app_monitoreo.tamano());

}