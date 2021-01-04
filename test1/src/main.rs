
struct Point{
    x:f32,
    y:f32,
}

enum e1{
    z1,
    z2(i32),
   z3{n:i32},
}

fn main() {
 

let t1=e1::z1;
let t2=e1::z2(10);
let t3=e1::z3{n:70};

match t1{
    e1::z1=>println!("=z1");
    e1::z2(10)=>println!("=z2");
    e1::z3{n:70}=>println!("=z3");
}
  
}

