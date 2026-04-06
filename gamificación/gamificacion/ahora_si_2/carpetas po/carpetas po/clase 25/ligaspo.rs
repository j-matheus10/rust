#[derive(Debug, Clone)]
struct Brigadista {
    nombre: String,
    edad: u8,
    puntos: i32,
    liga: String,
}
#[derive(Debug, Clone)]
struct Liga {
    nombre: String,
    superior: i32,
    inferior: i32,
    ponderador: f32,
    ranking: Vec<Brigadista>,
}

struct Monitoreo{
    ligas: Vec<Liga>,
}
impl Liga {
    fn new(n:String) -> Self {
        Liga {
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
        self.ranking.len()
    } 

}
impl Monitoreo{
    fn new() -> Self {
        Monitoreo {
            ligas: Vec::new(),
        }
    }

    fn crea_liga(&mut self, liga:Liga) {
        self.ligas.push(liga);
    }

    fn tamano(&self) -> usize {
        self.ligas.len()
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
    let mut bronce = Liga::new(String::from("Bronce"));
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
    bronce.crea_usuario(b1);
    println!("La cantidad de usuarios es {}", bronce.tamano());
    bronce.crea_usuario(b2);
    println!("La cantidad de usuarios es {}", bronce.tamano());
    app_monitoreo.crea_liga(bronce);
    println!("La cantidad de ligas es {}", app_monitoreo.tamano());


    let mut plata = Liga::new(String::from("Plata"));
    app_monitoreo.crea_liga(plata.clone());
    let mut b3 = Brigadista {
        nombre: String::from("Pepe"),
        edad: 20,
        puntos: 0,
        liga: String::from("Plata"),
    };
    let b3c = b3.clone();

        plata.crea_usuario(b3);
    let mut b4 = Brigadista {
        nombre: String::from("messi"),
        edad: 37,
        puntos: 100,
        liga: String::from("Plata"),
    };
    let b4c = b4.clone();

        plata.crea_usuario(b4);

   let mut b5 = Brigadista {
    nombre: String::from("El bicho"),
    edad: 40,
    puntos: 90,
    liga: String::from("Plata"),
   };
    let b5c = b5.clone();

    plata.crea_usuario(b5);


   println!("El tercer brigadista es {:?}", b3c);
   println!("El cuarto brigadista es {:?}", b4c);
   println!("El quinto brigadista es {:?}", b5c);
   println!("La cantidad de usuarios es {}", plata.tamano());
   println!("La cantidad de ligas es {}", app_monitoreo.tamano());
   println!("Los nombres de las ligas son {:?}",app_monitoreo.ligas);
}