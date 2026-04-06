fn main() {

    let l1:[i32;6] = [4, 5, 8 , 5, 14, 12];
    let l0:[i32;3] = [ 2, 4, 5];
    let mut rs:i32 = 0;

    for p in l0
    {
         for q in l1
         {
            if p == q
            {
                rs += 1;
            } 
         }
    }

    println! ("{}", rs);
}
