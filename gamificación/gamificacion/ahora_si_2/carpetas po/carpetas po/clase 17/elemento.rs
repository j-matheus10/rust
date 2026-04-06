fn main() {

let z:i32 = 10;
let l:[i32;5] = [10, 10, 10 , 0, 10];
let mut rs:i32 = 0;

for i in l 
{

    if z == l[i]
    {
        rs += 1;
    }
}

println! ("{}", rs);

}