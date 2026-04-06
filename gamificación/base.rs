  //J Matheus. M Jofre.  


    //Crear tipo de dato "Tarea", asignable a empleados y proyectos.
#[derive(Debug, Clone)] // Debug para poder imprimir con {:?} y Clone para poder asignar por separados a empleados y proyectos sin interferencias
struct Tarea {
    descripcion: String, //descripción de la tarea
    completada: bool, //True or false
    puntos: u32, //Puntos que sumara en el rankig al completar la tarea, se usa u32 para separar de funcion penitencia
    asignado_a: String, // Nombre del empleado asignado
    proyecto: String,
}


    //Crear tipo de dato "Empleado", asignable a gerentes y proyectos.
#[derive(Debug, Clone)]
struct Empleado {
    nombre: String, 
    nivel: u32, //nivel calculado a partir de la experiencia
    experiencia: u32, //sumatoria total de puntos obtenidos en tareas   
    tareas: Vec<Tarea>, //lista de tareas asignadas
}

impl Empleado {
    fn nuevo(nombre: &str) -> Self { //Toma el valor del str dado como referencia
        Empleado {
            nombre: nombre.to_string(), //Asigna ese valor con .to_string()
            nivel: 1,
            experiencia: 0,
            tareas: vec![], // Se crea vector que almacena las tareas
        }
    }

    fn asignar_tarea(&mut self, actividad: Tarea) { //Permite modificar la variable a la que se le asignará la tarea para almacenarla en su vector "Tareas"
        self.tareas.push(actividad); // Introduce la nueva tarea al final de lista
    }

    fn completar_tarea(&mut self, index: usize) {
        if let Some(actividad) = self.tareas.get_mut(index) { // if let para extraer el valor hallado por el get_mut(index) y lo asigna a la variable "Actividad" usada solo en esta funcion
            if !actividad.completada { // si la tarea NO está completada entonces:
                actividad.completada = true; // se asigna la actividad como completada
                self.experiencia += actividad.puntos; // se suman los puntos de al actividad a la experiencia del empleado  
                self.actualizar_nivel(); // se ejecuta la función actualizar nivel
            }
        }
    }

    fn actualizar_nivel(&mut self) {
        self.nivel = 1 + self.experiencia / 100; // 100 de experiencia por cada nivel
    }

    fn puntos_totales(&self) -> u32 {
        self.tareas
            .iter() // iterada todas las tareas del empleado
            .filter(|actividad| actividad.completada) // filtra solo las completadas
            .map(|actividad| actividad.puntos) // considera solo los puntos de las tareas filtradas
            .sum() // los suma y devuelve el total po
    }
}

#[derive(Debug)]
struct Proyecto {
    nombre: String,
    tareas: Vec<Tarea>,
}

impl Proyecto {
    fn nuevo(nombre: &str) -> Self {
        Proyecto {
            nombre: nombre.to_string(),
            tareas: vec![], // crea vector tareas vacío
        }
    }

    fn agregar_tarea(&mut self, actividad: Tarea) {
        self.tareas.push(actividad);
    }

    fn progreso(&self) -> f32 { //solo lee el objeto más no lo modifica y regresa un f32
        if self.tareas.is_empty() {
            0.0 // si no hay tareas el progreso es 0, por que no filtra por tareas completadas, es por el total.
        } else {
            let completadas = self.tareas.iter().filter(|actividades|actividades.completada).count(); // se crea la variable completadas 
            completadas as f32 / self.tareas.len() as f32 * 100.0 // se convierten la variable completadas a f32 e igual al valor de la cantidad de tareas y se dividen y luego se multiplica por 100
        }
    }
}

#[derive(Debug)]
struct Gerente {
    nombre: String,
    equipo: Vec<Empleado>,
    proyectos: Vec<Proyecto>,
}

impl Gerente {
    fn nuevo(nombre: &str) -> Self {
        Gerente {
            nombre: nombre.to_string(),
            equipo: vec![],
            proyectos: vec![],
        }
    }

    fn agregar_empleado(&mut self, sujeto: Empleado) {
        self.equipo.push(sujeto);
    }

    fn crear_proyecto(&mut self, plan: Proyecto) {
        self.proyectos.push(plan);
    }

    fn asignar_tarea(&mut self, nombre_a_buscar: &str,mut actividad:Tarea) {

    let sujeto = self.equipo.iter_mut().find(|l_empleados| l_empleados.nombre == nombre_a_buscar).expect("Trabajador no encontrado en la base de datos");
    actividad.asignado_a = Some(nombre_a_buscar.to_string());
    sujeto.tareas.push(actividad);
    }

    fn asignar_tarea_a_proyecto(&mut self, plan_idx: usize, actividad: Tarea) {
    if let Some(plan) = self.proyectos.get_mut(plan_idx) {
        plan.agregar_tarea(actividad);
        }
    }


    fn ver_progreso_proyectos(&self) {
        for p in &self.proyectos {
            println!(
                "📁 Proyecto: {} - Progreso: {:.1}%",
                p.nombre,
                p.progreso()
            );
        }
    }

    fn mostrar_estado_equipo(&self) {
        for e in &self.equipo {
            println!(
                "👤 {} | Nivel: {} | XP: {} | Puntos totales: {}",
                e.nombre,
                e.nivel,
                e.experiencia,
                e.puntos_totales()
            );
        }
    }

    fn ranking(&self) {
        let mut lista = self.equipo.clone();
        lista.sort_by_key(|e| std::cmp::Reverse(e.puntos_totales()));
        println!("\n🏆 Ranking de productividad:");
        for (i, e) in lista.iter().enumerate() {
            println!("{}. {} - {} puntos", i + 1, e.nombre, e.puntos_totales());
        }
    }
}

fn main() {
    let mut gerente = Gerente::nuevo("Jesús");

    // Agregar empleados
    gerente.agregar_empleado(Empleado::nuevo("Ana"));
    gerente.agregar_empleado(Empleado::nuevo("Carlos"));

    // Crear un proyecto
    gerente.crear_proyecto(Proyecto::nuevo("Lanzamiento Plataforma"));

    // Asignar tareas a través del gerente
    gerente.asignar_tarea_a_empleado(
        0,
        Tarea {
            descripcion: "Diseñar interfaz".to_string(),
            completada: false,
            puntos: 40,
            asignado_a: "Ana".to_string(),
        },
    );

    gerente.asignar_tarea_a_empleado(
        0,
        Tarea {
            descripcion: "Desarrollar backend".to_string(),
            completada: false,
            puntos: 60,
            asignado_a: "Carlos".to_string(),
        },
    );

    // Simular que Ana completa su tarea
    if let Some(ana) = gerente.equipo.iter_mut().find(|e| e.nombre == "Ana") {
        ana.completar_tarea(0);
    }

    // Mostrar estado general
    gerente.mostrar_estado_equipo();
    gerente.ver_progreso_proyectos();
    gerente.ranking();
}
