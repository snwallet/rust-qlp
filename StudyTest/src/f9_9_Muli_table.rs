//分别用whiel,loop,for实现99乘法表

//9*9乘法表while的实现
pub fn f9_9_show()
{
    let mut i:i32=1;
    let mut j:i32=1;
    let mut k:i32=1;
    println!("while循环实现的9*9乘法表");
    println!("**********");
    while k<10
    { 
        i=1;
        while i<k+1
        {
            print!("{}*{}={}  ",i,j,i*j);
            i=i+1;
            if i*j<10
            {
                print!(" ");
            }
        }
        println!("");
        k=k+1;
        j=j+1;
        
    }/**/
    println!("**********");
}

//9*9乘法表loop的实现
pub fn f9_9_show2()
{
    let mut i:i32=1;
    let mut j:i32=1;
    let mut k:i32=1;
    println!("loop循环实现的9*9乘法表");
    println!("**********");
    loop 
    { 
        if k>9
        {
            break;
        }

        i=1;
        loop 
        {
            if i>k
            {
                break;
            }
            print!("{}*{}={}  ",i,j,i*j);
            i=i+1;
            if i*j<10
            {
                print!(" ");
            }
        }
        println!("");
        k=k+1;
        j=j+1;
        
    }/**/
    println!("**********");
}

//9*9乘法表for的实现
pub fn f9_9_show3()
{
    let mut i:i32=1;
    let mut j:i32=1;
    let mut k:i32=1;
    println!("for循环实现的9*9乘法表");
    println!("**********");
    for k in 1..10
    { 
    //    i=1;
        for i in 1..k+1
        {
            print!("{}*{}={}  ",i,j,i*j);
         //   i=i+1;
            if i*j<10
            {
                print!(" ");
            }
        }
        println!("");
 //       k=k+1;
        j=j+1;
        
    }/**/
    println!("**********");
}