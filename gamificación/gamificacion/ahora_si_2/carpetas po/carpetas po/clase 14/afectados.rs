fn main () {

    let santiago: [f32;5] = [4.0, 5.0, 0.0, 10.0, 5.0];
    let melipilla: [f32;5] = [20.0, 30.0, 41.0, 32.0, 10.0];
    let curacavi: [f32;5] = [5.0, 8.0, 3.0, 10.0, 4.0];
    let tiltil: [f32;5] = [8.0, 4.0, 32.0, 25.0, 3.0];

    let mut sum_santiago:f32 = 0.0;
    let mut sum_melipilla:f32 = 0.0;
    let mut sum_curacavi:f32 = 0.0;
    let mut sum_tiltil:f32 = 0.0;
    
    let mut sum_total:f32 = 0.0;

    for i in 0..=4 {
        sum_santiago += santiago[i];
    }

    for i in 0..=4 {
        sum_melipilla += melipilla[i];
    }

    for i in 0..=4 {
        sum_curacavi += curacavi[i];
    }

    for i in 0..=4 {
        sum_tiltil += tiltil[i];
    }

    sum_total = sum_santiago + sum_melipilla + sum_melipilla + sum_tiltil;

println!("cantidad de afectados en santiago es de {}; en melipilla es de {}; en curacavi es de {}; y en tiltil es de {}.", sum_santiago,sum_melipilla,sum_curacavi,sum_tiltil);
println! ("Cantidad total de afectados es {}", sum_total);

}