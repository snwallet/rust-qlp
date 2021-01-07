//trait的各种测试应用

pub fn method1()
{
//   type untype=i32;
    trait instance
    {
      fn  SubMethod(&self)->i32;

    }
    
    impl instance for i32{
      fn  SubMethod(&self)->i32
        {
            let (mut a,mut b):(i32,i32)=(1,2);
            a=b+*self;
            return a;
        }
    }
    let mut temp:i32=55;
    let mut temp2:i32;
    let mut result= temp.SubMethod();
  //  let mut result=instance::SubMethod(&temp);
    print!("{}\n",result);
}

pub trait undefType1<T1,T2,T3>
{
    type userdef;
   fn Method1(&self,v1:Self::userdef,v2:T1)->(T2,T3);
}
impl undefType1<i32,i32,i32> for i32
{
    type userdef=i32;
    fn Method1(&self,v1:Self::userdef,v2:i32)->(i32,i32)
    {
        return (*self+v1,v2);
    }
}
pub fn funcation1()
{
    let mut temp:i32=90;
    let mut temp2:(i32,i32,i32)=(90,200,270);
  //  let mut result:(i32,i32)=temp1.Method1();
    let mut result:(i32,i32)=undefType1::Method1(&90,190,200);

    print!("{:?}",result);

}