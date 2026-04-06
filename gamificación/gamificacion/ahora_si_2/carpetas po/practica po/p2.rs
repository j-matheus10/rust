//Crea una función llamada contar_ocurrencias que reciba un vector de enteros y un número objetivo, y devuelva cuántas veces aparece ese número en la lista.

fn main(){

    let u:[i32;6] = [1, 2, 3, 5, 5, 6];
    let obj:i32 = 5;

    println! ("Para el vector {:?} la cantidad de veces que se repite {} es {}", 
    u,
    obj,
    fr2(&u,obj));
}

fn fr2(z: &[i32], goal: i32) -> i32 {

    let aco:i32 = z.iter().copied().filter(|x| *x == goal).count() as i32;
    return aco;
}