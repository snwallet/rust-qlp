//Box指针及"..="运算符使用
pub fn funcation1()
{
    let mut ptr1:Box<i32>=Box::new(10);
//    print!("{}\n",ptr1);
    
    for i in 0..=10
    {
        print!("{} ",i);
    }
}

//以下实现的功能为：
//输入一个参数时，求以此参数为半径的圆的面积
//输入两个参数时，则求矩形面积


pub trait defType
{
    fn to_type(self)->f32;

}
impl defType for i32
{
    fn to_type(self)->f32
    {
        return ((self*self) as f32)*3.14;
    }

}

impl defType for f32
{
    fn to_type(self)->f32
    {
        return ((self*self) as f32)*3.14;
    }

}

impl defType for (i32,i32)
{
    fn to_type(self)->f32
    {
        return (self.0*self.1) as f32;
    }

}

impl defType for (i32,f32)
{
    fn to_type(self)->f32
    {
        return (self.0 as f32)*self.1;
    }

}

impl defType for (f32,i32)
{
    fn to_type(self)->f32
    {
        return self.0*(self.1 as f32);
    }

}

impl defType for (f32,f32)
{
    fn to_type(self)->f32
    {
        return self.0*self.1;
    }

}


pub fn getAera<T:defType>(x:T)
{
   let temp= x.to_type();
    print!("area={:?}",temp);
}
