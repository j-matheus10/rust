fn main(){

    let l:[i32;4] = [ 2, 4, 5, 10];
    let o:[i32;6] = [ 5, 5, 5, 1, 5, 4];

    println! (" La cantidad mayor  en {:?} es {}", l, mayor_lista(l.to_vec()));
    println! (" La cantidad mayor  en {:?} es {}", o, mayor_lista(o.to_vec()));
    println! (" La cantidad menor  en {:?} es {}", l, menor_lista(l.to_vec()));
    println! (" La cantidad menor  en {:?} es {}", o, menor_lista(o.to_vec()));
    println! (" La suma de {:?} es {}", l, suma_lista(l.to_vec()));
}

fn mayor_lista(z:Vec<i32>) -> i32 {

    let mut acc:i32 = z[0];

    for p in z {
        if p > acc {
                acc = p;
        }
    }
    return acc;
}

fn menor_lista(z:Vec<i32>) -> i32 {

    let mut acc:i32 = z[0];

    for p in z {
        if p < acc {
                acc = p;
        }
    }
    return acc;
}

fn suma_lista(z:Vec<i32>) -> i32 {

    let mut acc:i32 = 0;

    for p in z {

        acc += p;
    }

    return acc;
}