//各种指针用法测试
pub fn fn_temp2()
{
    let mut n=10;
    let mut m=1;

    let mut p1=&n;
    let mut p2=&mut n;
    let mut p3:Box<i32>=Box::new(90);

    *p2=30;
    
 //   println!("p1={}",*p1);
 //   println!("n={}",n);



    let mut p4:*const i32=&n;
    let mut p5:*mut i32=&mut n;
    unsafe{
        
        *p5=*p4+1;
    }
    println!("n={}",n);

/*
    let mut array:[i32;5]=[1,2,3,4,5];
    let mut p6:*mut [i32;5]=&mut array as *mut [i32;5];

    for i in 0..5
    {
        unsafe
        {
            
        }
    }*/
}