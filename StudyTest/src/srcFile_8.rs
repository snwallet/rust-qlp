
pub fn function1()
{
    macro_rules!  userdef{
        ($($value:expr),*) => 
        {
            $(print!("{:?}\t",$value);)*
            
        };
    }
    userdef!(10,123,String::from("ABC"));
}

pub fn function2()
{
    macro_rules!  getArea{
        
        ($r:expr) =>
         {
            ($r as f32)*($r as f32)*3.14
        };
        ($a:expr,$b:expr)=>
        {
            ($a as f32)*($b as f32)
        };

    /*    ($($x:expr),*) =>
        {
            
         //  let mut temp:f32=1.0;
         //  $(temp=temp*($x as f32))*
         
          
       };*/


    }

    let mut area:f32;
    area=getArea!(2,15);
    print!("{}",area);





}