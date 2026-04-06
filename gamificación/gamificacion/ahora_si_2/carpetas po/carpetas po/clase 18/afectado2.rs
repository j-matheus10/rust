fn main (){
    let ids:[i32;4] = [10, 4, 8 ,20];
    let stock:[i32;4] = [120, 4, 45, 60];
    let mut suma:i32 = 0;
    let user:[i32;2] = [4, 20];

 for p in user{
    suma+= busca(ids.to_vec(), stock.to_vec(), p);
}
println!("El total de stock de productos{:?} es {}", user, suma)

fn busca(id:Vec<i32>, st:Vec<i32>, p:i32)-> i32{
    let mut acc:i32 = 0;
    for i in user{
        for j in 0..ids.len(){
            if i == ids[j]{
                acc+= stock[j]
            }
        }
    }
}
}