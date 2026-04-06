fn main(){

    let l:[i32;4] = [ 2, 4, 5, 10];
   
    println! (" La cantidad mayor  en {:?} es {}", l, mayor_lista(l.to_vec()));
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