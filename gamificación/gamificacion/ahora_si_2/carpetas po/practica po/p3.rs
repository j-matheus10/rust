fn main () {

    let u:[i32;7] = [12, 13, 14, 15, 16, 17, 18];

    println!("El vector {:?} escrito al revés sería {:?}",
u,
trs(&u));
}

fn traspuesto(z: &[i32]) -> Vec<i32> {
    let mut invertido = Vec::with_capacity(z.len());
    for &item in z.iter().rev(){
        invertido.push(item);
    }
    return invertido;
}

fn trs(z: &[i32]) -> Vec<i32> {
    let inv = z.iter().rev().copied().collect();
    inv
}