use std::fmt::Debug;
//函数的泛型
pub fn funcation1<T1:Debug,T2:Debug>(a:T1,b:T2)
{
    print!("{:#?} {:#?}",a,b);

}
//结构体及结构元组的泛型
pub fn funcation2()
{
    struct stru1<T1,T2>
    where T1:Debug,T2:Debug
    {
        a:T1,
        b:T2
    }

    struct stru_tuple<T1,T2>(T1,T2);

    let mut s1:stru1<i32,i32>=stru1{a:90,b:180};
    let mut t1:stru_tuple<char,String>=stru_tuple('a',String::from("Victory!"));

    print!("{} {}\n",s1.a,s1.b);
    print!("{} {}\n",t1.0,t1.1);



}
    //数组指针的获取
pub fn funcation3()
{

    let mut array:[i32;10]=[8;10];
    let mut ptr:*mut [i32;10]=&mut array as *mut [i32;10];

    unsafe
    {
        let mut ptr2:*mut i32=ptr as *mut i32;
        let mut temp:*mut i32;

        for i in 0..10
        {   temp=((ptr2 as usize)+4*i) as *mut i32;
            print!("{} ",*temp);
        }

    }
    
}

//字符串使用方法测试
pub fn funcation4()
{
    let mut str1:&str="hello!";
    print!("{}\n",str1);

    let mut str2:String="Successful".to_string();
    str2.insert(str2.len(), '!');
    print!("{}\n",str2);
}

