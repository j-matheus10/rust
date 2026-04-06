//Crea una función promedio_lista que reciba un Vec<i32> y devuelva el promedio como f32.
fn main(){

    let u:[i32;4] = [0, 0, 0, 0];

    println!("El promedio de {:?} es {}", u, promedio_lista(&u))
}

fn promedio_lista(z:&[i32]) -> f32 {

    if z.is_empty(){
        println! ("Vector vacío");

        return 0.0;
    }
    
    if z.iter().all(|&x| x == 0){
        println!("Vector nulo (todos valores son 0)");
        return 0.0;
         
    }
     
    let mut acc:f32 = 0.0;

    for p in z 
    {
        acc += *p as f32;
    }

    let mean = acc as f32 /z.len() as f32;
    return mean;

}