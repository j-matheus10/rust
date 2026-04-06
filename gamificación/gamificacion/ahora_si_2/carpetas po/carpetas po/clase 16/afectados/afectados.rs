fn main(){
let stgo:[i32;5]=[4, 5, 0, 10, 5];
let meli:[i32;5]=[20, 30, 41, 32, 10];
let mut acumulador:i32 = 0;
let mut acumulador_2:i32 =0;
let umbral:i32 = 5;
for i in 0..=4{
    if stgo[i]>=umbral && meli[i] >= umbral{
        acumulador += stgo[i] + meli[i];
}
} 
println!["{}", acumulador];


for i in 0..=4{ 
    if stgo[i]>=umbral{
        acumulador_2 +=1}
    
    if meli[i]>=umbral{
        acumulador_2+=1
    }
}
println!["{}", acumulador_2]
}








