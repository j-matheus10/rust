fn main(){

    let l:[i32;4] = [2, 4, 5, 10];
    let m:i32 = 5;
    let o:[i32;6] = [5, 5, 3, 1, 5, 4];
    println! ("La cantidad de v menores que {} en {:?} es {}", m, l, cantidad_menor_a(m, l.to_vec()));
    println! ("La cantidad de v menores que {} en {:?} es {}", m, o, cantidad_menor_a(m, o.to_vec()));
    println! ("La cantidad de v mayores que {} en {:?} es {}", m, o, cantidad_mayor_a(m, o.to_vec()));
    println! ("La cantidad de v mayores que {} en {:?} es {}", m, l, cantidad_mayor_a(m, l.to_vec()));
}

fn cantidad_menor_a (k:i32, z:Vec<i32>) -> i32 {

    let mut acc:i32 = 0;
    for p in z {
        if p < k {
            acc += 1;
        }
    }
    return acc;
}

fn cantidad_mayor_a (k:i32, z:Vec<i32>) -> i32 {

    let mut acc:i32 = 0;
    for p in z {
        if p > k {
            acc +=1;
        }
    }
    return acc;
}