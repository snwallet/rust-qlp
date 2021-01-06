//冒泡排序法实现
pub fn num_sort()
{
   let mut array:[i32;10]=[10,8,3,67,38,90,100,32,2,8];
    let mut i:u16=1;
    let mut j:u16=1;
    let mut k:u16=1;
    let mut max:i32=array[0];
    let mut temp:i32=0;
    for i in 0..9
    {
        max=array[0];
        k=0;
        for j in 0..10-i
        {
            if max<array[j]
            {
                max=array[j];
                k=j as u16;
            }
        }

        temp=array[9-i as usize];
        array[9-i as usize]=max;
        array[k as usize]=temp;
 //       print!("{}  ",max);
    }
    //    println!("");
    for i in 0..10
    {
        print!("{}  ",array[i]);
    }
}

//切片的使用
pub fn fn_temp()
{
    let mut array:[i32;10]=[0,1,2,3,4,5,6,7,8,9];
    let mut array2=&array[2..5];
    let mut array3:Vec<i32>=vec![1];
    for i in array2.iter()
    {
        print!("{} ",i);
    }
    println!("");
    for i in 0..3
    {
        print!("{} ",array2[i]);
    }
    println!("");
    array3.push(2);
/*    for i in 0..12
    {
        array3.push(32);
     //   print!("{}",array3[i]);
    }
*/

}
