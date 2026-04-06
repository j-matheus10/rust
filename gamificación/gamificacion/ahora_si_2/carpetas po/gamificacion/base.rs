use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

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

    fn idx_tarea(&self, descripcion: &str) -> Option<usize> {
        self.tareas.iter().position(|t| t.borrow().descripcion == descripcion)
    }

    fn asignar_tarea(&mut self, tarea: TareaRc) {
        self.tareas.push(tarea);
    }

    fn completar_tarea(&mut self, index: usize) {
        if let Some(actividad) = self.tareas.get(index) {
            let mut tarea = actividad.borrow_mut();
            if !tarea.completada {
                tarea.completada = true;
                let puntos = tarea.puntos;
                drop(tarea); // liberar borrow_mut antes de mutar self
                self.experiencia += puntos;
                self.actualizar_nivel();
                println!("Tarea completada.");
            } else {
                println!("La tarea ya estaba completada.");
            }
        } else {
            println!("Índice de tarea inválido.");
        }
    }

    fn actualizar_nivel(&mut self) {
        self.nivel = 1 + self.experiencia / 100;
    }

    fn puntos_totales(&self) -> u32 {
        self.tareas
            .iter()
            .filter(|t| t.borrow().completada)
            .map(|t| t.borrow().puntos)
            .sum()
    }

    fn listar_tareas(&self) {
        println!("Tareas de {}:", self.nombre);
        for (i, tarea) in self.tareas.iter().enumerate() {
            let t = tarea.borrow();
            println!(
                "{}. [{}] {} - Proyecto: {}",
                i + 1,
                if t.completada { "X" } else { " " },
                t.descripcion,
                t.proyecto.as_deref().unwrap_or("Sin proyecto")
            );
        }
    }
}

#[derive(Debug)]
struct Proyecto {
    nombre: String,
    tareas: Vec<TareaRc>,
}

impl Proyecto {
    fn nuevo(nombre: &str) -> Self {
        Proyecto {
            nombre: nombre.to_string(),
            tareas: vec![],
        }
    }

    fn agregar_tarea(&mut self, tarea: TareaRc) {
        self.tareas.push(tarea);
    }

    fn listar_tareas(&self) {
        println!("Tareas del proyecto '{}':", self.nombre);
        for (i, tarea) in self.tareas.iter().enumerate() {
            let t = tarea.borrow();
            println!(
                "{}. [{}] {} - Asignado a: {}",
                i + 1,
                if t.completada { "X" } else { " " },
                t.descripcion,
                t.asignado_a
            );
        }
    }

    fn progreso(&self) -> f32 {
        if self.tareas.is_empty() {
            0.0
        } else {
            let completadas = self.tareas.iter().filter(|t| t.borrow().completada).count();
            completadas as f32 / self.tareas.len() as f32 * 100.0
        }
    }
}

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

    fn agregar_empleado(&mut self, empleado: Empleado) {
        self.equipo.push(empleado);
    }

    fn crear_proyecto(&mut self, proyecto: Proyecto) {
        self.proyectos.push(proyecto);
    }

    fn asignar_tarea(&mut self, nombre_empleado: &str, tarea: TareaRc, nombre_proyecto: Option<&str>) {
        if let Some(empleado) = self.equipo.iter_mut().find(|e| e.nombre == nombre_empleado) {
            tarea.borrow_mut().asignado_a = nombre_empleado.to_string();
            empleado.tareas.push(Rc::clone(&tarea));
        } else {
            println!("Empleado '{}' no encontrado.", nombre_empleado);
            return;
        }

        if let Some(nombre) = nombre_proyecto {
            if let Some(proyecto) = self.proyectos.iter_mut().find(|p| p.nombre == nombre) {
                proyecto.tareas.push(Rc::clone(&tarea));
            } else {
                println!("Proyecto '{}' no encontrado.", nombre);
            }
        }
    }

    fn idx_proyecto(&self, nombre: &str) -> Option<usize> {
        self.proyectos.iter().position(|p| p.nombre == nombre)
    }

    fn idx_empleado(&self, nombre: &str) -> Option<usize> {
        self.equipo.iter().position(|e| e.nombre == nombre)
    }

    fn mostrar_estado_equipo(&self) {
        println!("Estado del equipo:");
        for e in &self.equipo {
            println!(
                "{} | Nivel: {} | XP: {} | Puntos totales: {}",
                e.nombre,
                e.nivel,
                e.experiencia,
                e.puntos_totales()
            );
        }
    }

    fn ver_progreso_proyectos(&self) {
        println!("Progreso de proyectos:");
        for p in &self.proyectos {
            println!("{} - {:.1}%", p.nombre, p.progreso());
        }
    }

    fn ranking(&self) {
        let mut lista = self.equipo.clone();
        lista.sort_by_key(|e| std::cmp::Reverse(e.puntos_totales()));
        println!("\nRanking de productividad:");
        for (i, e) in lista.iter().enumerate() {
            println!("{}. {} - {} puntos", i + 1, e.nombre, e.puntos_totales());
        }
    }
}

fn leer_linea() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer");
    input.trim().to_string()
}

fn main() {
    let mut gerente = Gerente::nuevo("Jesús");

    loop {
        println!("\n===== Menú Principal =====");
        println!("1) Agregar empleado");
        println!("2) Crear proyecto");
        println!("3) Crear y asignar tarea");
        println!("4) Completar tarea de un empleado");
        println!("5) Mostrar estado del equipo");
        println!("6) Mostrar progreso de proyectos");
        println!("7) Mostrar ranking");
        println!("8) Listar tareas de un proyecto");
        println!("9) Listar tareas de un empleado");
        println!("10) Listar todos los proyectos");
        println!("11) Asignar tarea existente a un proyecto");

        println!("0) Salir");
        print!("Seleccione una opción: ");
        io::stdout().flush().unwrap();

        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                print!("Ingrese nombre del empleado: ");
                io::stdout().flush().unwrap();
                let nombre = leer_linea();
                gerente.agregar_empleado(Empleado::nuevo(&nombre));
                println!("Empleado '{}' agregado.", nombre);
            }
            "2" => {
                print!("Ingrese nombre del proyecto: ");
                io::stdout().flush().unwrap();
                let nombre = leer_linea();
                gerente.crear_proyecto(Proyecto::nuevo(&nombre));
                println!("Proyecto '{}' creado.", nombre);
            }
            "3" => {
                print!("Ingrese descripción de la tarea: ");
                io::stdout().flush().unwrap();
                let descripcion = leer_linea();

                print!("Ingrese puntos de la tarea: ");
                io::stdout().flush().unwrap();
                let puntos: u32 = leer_linea().parse().unwrap_or(0);

                print!("Asignar a empleado (nombre): ");
                io::stdout().flush().unwrap();
                let empleado = leer_linea();

                print!("Asignar a proyecto (nombre) o dejar vacío: ");
                io::stdout().flush().unwrap();
                let proyecto_input = leer_linea();
                let proyecto = if proyecto_input.is_empty() { None } else { Some(proyecto_input.as_str()) };

                let tarea = Rc::new(RefCell::new(Tarea {
                    descripcion,
                    completada: false,
                    puntos,
                    asignado_a: "".to_string(),
                    proyecto: proyecto.map(|s| s.to_string()),
                }));

                gerente.asignar_tarea(&empleado, tarea, proyecto);
                println!("Tarea asignada.");
            }
            "4" => {
                print!("Ingrese nombre del empleado: ");
                io::stdout().flush().unwrap();
                let empleado = leer_linea();

                if let Some(idx_empleado) = gerente.idx_empleado(&empleado) {
                    let emp = &mut gerente.equipo[idx_empleado];
                    emp.listar_tareas();

                    print!("Ingrese número de tarea a completar: ");
                    io::stdout().flush().unwrap();
                    if let Ok(idx_tarea) = leer_linea().parse::<usize>() {
                        if idx_tarea == 0 || idx_tarea > emp.tareas.len() {
                            println!("Número de tarea inválido.");
                        } else {
                            emp.completar_tarea(idx_tarea - 1);
                        }
                    } else {
                        println!("Entrada inválida.");
                    }
                } else {
                    println!("Empleado no encontrado.");
                }
            }
            "5" => gerente.mostrar_estado_equipo(),
            "6" => gerente.ver_progreso_proyectos(),
            "7" => gerente.ranking(),
            "8" => {
                print!("Ingrese nombre del proyecto: ");
                io::stdout().flush().unwrap();
                let proyecto = leer_linea();

                if let Some(idx) = gerente.idx_proyecto(&proyecto) {
                    gerente.proyectos[idx].listar_tareas();
                } else {
                    println!("Proyecto no encontrado.");
                }
            }
            "9" => {
                print!("Ingrese nombre del empleado: ");
                io::stdout().flush().unwrap();
                let empleado = leer_linea();

                if let Some(idx) = gerente.idx_empleado(&empleado) {
                    gerente.equipo[idx].listar_tareas();
                } else {
                    println!("Empleado no encontrado.");
                }
            }
            "10" => {
                if gerente.proyectos.is_empty() {
                println!("No hay proyectos creados.");
                } else {
                println!("Proyectos:");
                for (i, p) in gerente.proyectos.iter().enumerate() {
                println!("{}. {}", i + 1, p.nombre);
                }
                }
            }

            "11" => {
    print!("Ingrese nombre del empleado dueño de la tarea: ");
    io::stdout().flush().unwrap();
    let empleado = leer_linea();

    // Buscar índice empleado primero, sin tomar referencias mutables aún
    if let Some(idx_emp) = gerente.idx_empleado(&empleado) {
        // Mostrar tareas
        // Esto es seguro porque solo es inmutable y no tomamos &mut aún
        {
            let emp = &gerente.equipo[idx_emp];
            emp.listar_tareas();
        }

        print!("Ingrese número de tarea a asignar a un proyecto: ");
        io::stdout().flush().unwrap();
        if let Ok(idx_tarea) = leer_linea().parse::<usize>() {
            if idx_tarea == 0 {
                println!("Número de tarea inválido.");
                return;
            }

            print!("Ingrese nombre del proyecto al que quiere asignar la tarea: ");
            io::stdout().flush().unwrap();
            let proyecto = leer_linea();

            // Buscar índice del proyecto antes de mutar anything
            if let Some(idx_proy) = gerente.idx_proyecto(&proyecto) {
                // Ahora sí tomamos mutable el empleado y el proyecto para modificar
                let tarea_rc = {
                    let emp = &mut gerente.equipo[idx_emp];
                    if idx_tarea > emp.tareas.len() {
                        println!("Número de tarea inválido.");
                        return;
                    }
                    Rc::clone(&emp.tareas[idx_tarea - 1])
                };

                let proyecto_ref = &mut gerente.proyectos[idx_proy];

                // Verificar si tarea ya asignada
                let ya_asignada = proyecto_ref.tareas.iter()
                    .any(|t| Rc::ptr_eq(t, &tarea_rc));

                if ya_asignada {
                    println!("La tarea ya está asignada a este proyecto.");
                } else {
                    proyecto_ref.agregar_tarea(Rc::clone(&tarea_rc));
                    tarea_rc.borrow_mut().proyecto = Some(proyecto.clone());
                    println!("Tarea asignada al proyecto '{}'.", proyecto);
                }
            } else {
                println!("Proyecto no encontrado.");
            }
        } else {
            println!("Entrada inválida.");
        }
    } else {
        println!("Empleado no encontrado.");
    }
}


            "0" => {
                println!("Saliendo...");
                break;
            }
            _ => println!("Opción inválida."),
        }
    }
}
