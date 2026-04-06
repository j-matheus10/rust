fn main(){

    let mut x:i32 = 10;
    let mut y:i32 = x;

    println!("El valor de x es {}.", x);
    println!("El valor de y es {}.", y);

    x = 30;
    y += 10;

    println!("El valor de x es {}.", x);
    println!("El valor de y es {}.", y);

    let s1= String::from("Hola");
    let s2 = s1.clone();
    
    println!("El valor de s1 es {}.", s1);
    println!("El valor de s2 es {}.", s2);

    println!("{}", fx(x,y));
    println!("{}", fx2(x,y));
    println!("{}", fx3(&mut x,y));
    println!("{}",x);
}


fn fx(z:i32, w:i32) -> i32 {

    let m = z + w;
    m
}

fn fx2(mut z:i32, w:i32) -> i32 {

    z += w;
    z
}

fn fx3(z:&mut i32, w:i32) -> i32 {
    *z += w;
    *z
}