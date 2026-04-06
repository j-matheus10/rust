// J Matheus. M Jofre.
use std::cell::RefCell;
use std::rc::Rc;
use std::io;

type TareaRc = Rc<RefCell<Tarea>>;

#[derive(Debug, Clone)]
struct Tarea {
    descripcion: String,
    completada: bool,
    puntos: u32,
    asignado_a: String,
    proyecto: Option<String>,
}

#[derive(Debug, Clone)]
struct Empleado {
    nombre: String,
    nivel: u32,
    experiencia: u32,
    tareas: Vec<TareaRc>,
}

impl Empleado {
    fn nuevo(nombre: &str) -> Self {
        Empleado {
            nombre: nombre.to_string(),
            nivel: 1,
            experiencia: 0,
            tareas: vec![],
        }
    }

    fn idx_tarea(&self, descripcion: &str) -> usize {
        self.tareas
            .iter()
            .position(|t| t.borrow().descripcion == descripcion)
            .expect("Tarea no encontrada en el empleado")
    }

    fn asignar_tarea(&mut self, actividad: TareaRc) {
        self.tareas.push(actividad);
    }

    fn completar_tarea(&mut self, index: usize) {
        let mut debe_actualizar = false;

        if let Some(actividad) = self.tareas.get(index) {
            let mut tarea = actividad.borrow_mut();
            if !tarea.completada {
                tarea.completada = true;
                self.experiencia += tarea.puntos;
                debe_actualizar = true;
            }
        }

        if debe_actualizar {
            self.actualizar_nivel();
        }
    }

    fn actualizar_nivel(&mut self) {
        self.nivel = 1 + self.experiencia / 100;
    }

    fn puntos_totales(&self) -> u32 {
        self.tareas
            .iter()
            .filter(|actividad| actividad.borrow().completada)
            .map(|actividad| actividad.borrow().puntos)
            .sum()
    }

    fn listar_tareas(&self) {
        for (i, tarea) in self.tareas.iter().enumerate() {
            let tarea = tarea.borrow();
            println!(
                "{}. [{}] {} - Proyecto: {}",
                i + 1,
                if tarea.completada { "X" } else { " " },
                tarea.descripcion,
                tarea.proyecto.clone().unwrap_or("Sin proyecto".to_string())
            );
        }
    }
}

#[derive(Debug)]
struct Proyecto {
    nombre: String,
    tareas: Vec<TareaRc>,
    empleados: Vec<String>,
}

impl Proyecto {
    fn nuevo(nombre: &str) -> Self {
        Proyecto {
            nombre: nombre.to_string(),
            tareas: vec![],
            empleados: vec![],
        }
    }

    fn agregar_tarea(&mut self, actividad: TareaRc) {
        self.tareas.push(actividad);
    }

    fn listar_tareas(&self) {
        for (i, tarea) in self.tareas.iter().enumerate() {
            let tarea = tarea.borrow();
            println!(
                "{}. [{}] {} - Asignado a: {}",
                i + 1,
                if tarea.completada { "X" } else { " " },
                tarea.descripcion,
                tarea.asignado_a
            );
        }
    }

    fn progreso(&self) -> f32 {
        if self.tareas.is_empty() {
            0.0
        } else {
            let completadas = self
                .tareas
                .iter()
                .filter(|t| t.borrow().completada)
                .count();
            completadas as f32 / self.tareas.len() as f32 * 100.0
        }
    }

    fn agregar_empleado(&mut self, nombre: &str) {
        if !self.empleados.contains(&nombre.to_string()) {
            self.empleados.push(nombre.to_string());
        }
    }

    fn listar_empleados(&self) {
        println!("Empleados en el proyecto '{}':", self.nombre);
        for (i, nombre) in self.empleados.iter().enumerate() {
            println!("{}. {}", i + 1, nombre);
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

    fn asignar_tarea(&mut self, nombre_empleado: &str, tarea: TareaRc, nombre_proyecto: Option<&str>) {
        if let Some(empleado) = self.equipo.iter_mut().find(|e| e.nombre == nombre_empleado) {
            tarea.borrow_mut().asignado_a = nombre_empleado.to_string();
            empleado.tareas.push(Rc::clone(&tarea));
        }

        if let Some(nombre) = nombre_proyecto {
            if let Some(proyecto) = self.proyectos.iter_mut().find(|p| p.nombre == nombre) {
                proyecto.tareas.push(Rc::clone(&tarea));
                proyecto.agregar_empleado(nombre_empleado);
            }
        }
    }

    fn idx_proyecto(&self, nombre: &str) -> usize {
        self.proyectos
            .iter()
            .position(|p| p.nombre == nombre)
            .expect("Proyecto no encontrado, créalo")
    }

    fn idx_empleado(&self, nombre: &str) -> usize {
        self.equipo
            .iter()
            .position(|p| p.nombre == nombre)
            .expect("Trabajador no encontrado, invítalo a tu equipo!")
    }

    fn rt_proyecto(&mut self, nombre_empleado: &str, idx_tarea: usize, nombre_proyecto: &str) {
        let tarea = {
            let empleado = self
                .equipo
                .iter()
                .find(|e| e.nombre == nombre_empleado)
                .expect("Empleado no encontrado");
            Rc::clone(&empleado.tareas[idx_tarea])
        };

        if let Some(proyecto) = self.proyectos.iter_mut().find(|p| p.nombre == nombre_proyecto) {
            proyecto.agregar_tarea(Rc::clone(&tarea));
            proyecto.agregar_empleado(nombre_empleado);
        }
    }

    fn ver_progreso_proyectos(&self) {
        for p in &self.proyectos {
            println!(" Proyecto: {} - Progreso: {:.1}%", p.nombre, p.progreso());
        }
    }

    fn mostrar_estado_equipo(&self) {
        for e in &self.equipo {
            println!(
                " {} | Nivel: {} | XP: {} | Puntos totales: {}",
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
        println!("\n Ranking de productividad:");
        for (i, e) in lista.iter().enumerate() {
            println!("{}. {} - {} puntos", i + 1, e.nombre, e.puntos_totales());
        }
    }
}

fn main() {
    let mut gerente = Gerente::nuevo("Jesús");

    gerente.agregar_empleado(Empleado::nuevo("Ana"));
    gerente.agregar_empleado(Empleado::nuevo("Carlos"));
    gerente.crear_proyecto(Proyecto::nuevo("Lanzamiento Plataforma"));

    let tarea1 = Rc::new(RefCell::new(Tarea {
        descripcion: "Diseñar interfaz".to_string(),
        completada: false,
        puntos: 120,
        asignado_a: "".to_string(),
        proyecto: Some("Lanzamiento Plataforma".to_string()),
    }));
    gerente.asignar_tarea("Ana", Rc::clone(&tarea1), Some("Lanzamiento Plataforma"));

    let tarea2 = Rc::new(RefCell::new(Tarea {
        descripcion: "Desarrollar backend".to_string(),
        completada: false,
        puntos: 60,
        asignado_a: "".to_string(),
        proyecto: Some("Lanzamiento Plataforma".to_string()),
    }));
    gerente.asignar_tarea("Carlos", Rc::clone(&tarea2), Some("Lanzamiento Plataforma"));

    if let Some(ana) = gerente.equipo.iter_mut().find(|e| e.nombre == "Ana") {
        ana.completar_tarea(0);
    }

    gerente.mostrar_estado_equipo();
    gerente.ver_progreso_proyectos();
    gerente.ranking();

    let tarea3 = Rc::new(RefCell::new(Tarea {
        descripcion: "FODA".to_string(),
        completada: false,
        puntos: 90,
        asignado_a: "".to_string(),
        proyecto: None,
    }));
    gerente.asignar_tarea("Ana", Rc::clone(&tarea3), Some("Lanzamiento Plataforma"));

    let idx = gerente.idx_proyecto("Lanzamiento Plataforma");
    gerente.proyectos[idx].listar_tareas();
    gerente.proyectos[idx].listar_empleados();

    println!("\nPresiona ENTER para salir...");
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);

}
