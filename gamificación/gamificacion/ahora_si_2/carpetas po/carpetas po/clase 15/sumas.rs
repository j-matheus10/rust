fn main(){

    let stgo: [f32;5] = [4.0, 5.0, 0.0, 10.0, 5.0];
    let mel: [f32;5] = [ 20.0, 30.0, 41.0, 32.0, 10.0];
    let mut acm:f32 = 0.0;
    let mut sum:f32 = 0.0;
    let umbral:f32 = 5.0;

    for i in 0..=4 {

        if stgo[i]>= umbral && mel[i]>= umbral
        { 
        acm += stgo[i] + mel[i];
        }
    } 
    for i in 0..=4 {

        if stgo[i]>= umbral
        { 
        sum += 1.0;
        }
    } 
    
    for i in 0..=4 {
    
        if mel[i]>= umbral  
        { 
        sum += 1.0;
        }
    } 
println! ("{}", acm);
println! ("{}", sum);

}