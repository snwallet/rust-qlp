


fn main() {
 
   let x:i32=20;
   let y:i32=35;
   let max:i32=CompareValue(x,y);
  println!("max={}",max);
}

fn CompareValue(v1:i32,v2:i32)->i32
{
    if v1>v2
    {
        return v1;
    }else
    {
        return v2;
    }
}
