fn main (){
    let ids:[i32;4] = [10, 4, 8 ,20];
    let stock:[i32;4] = [120, 4, 45, 60];
    let mut acc:i32 = 0;
    let user:[i32;2] = [4, 20];


    for i in user{
        for j in 0..ids.len(){
            if i == ids[j]{
                acc+= stock[j]
            }
        }
    }
        
println!("La suma solicitada de productos {:?}, fue de {}", user, acc)
        
      
    
}
