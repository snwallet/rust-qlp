//tuple
pub fn tupleUse()
{
    let mut tu1:(i32,f32,char,String);
    tu1=(19,10.1,'a',String::from("abcd"));
  
    print!("{} ",tu1.0);
    print!("{} ",tu1.1);
    print!("{} ",tu1.2);
    print!("{} ",tu1.3);
    println!("");

    //元组的分解
    let  (mut a,mut b,mut c,mut d):(i32,f32,char,String)=tu1;
    print!("{} ",a);
    print!("{} ",b);
    print!("{} ",c);
    print!("{} ",d);
    println!("");

    //元组重新赋值
    tu1=(100,78.2,'z',String::from("victory"));
    println!("{:?}",tu1);

}
//结构体
pub fn structUse()
{
    struct stru1
    {
        a:char,
        b:String,
        c:i32,
    }

    let mut s1:stru1;
    s1=stru1{a:'n',b:String::from("abcdef"),c:30};
    print!("{} ",s1.a);
    print!("{} ",s1.b);
    print!("{} ",s1.c);

}
//tuple and struct
pub fn structAndtuple()
{
    struct st(i32,char,String);
    let mut s1:st;

    s1=st(1,'c',String::from("abc"));

    print!("{} ",s1.0);
    print!("{} ",s1.1);
    print!("{} \n\n",s1.2);

    s1=st(2,'z',String::from("btf"));
    print!("{} ",s1.0);
    print!("{} ",s1.1);
    print!("{} \n\n",s1.2);

}

//enum
pub fn enumUse()
{
    enum en
    {
        key,
        event1(char),
        event2(char,String,i32),
       event3{a:String,b:char},
    }

    let (mut e1,mut e2,mut e3,mut e4):(en,en,en,en);
    e1=en::key;
    e2=en::event1('a');
    e3=en::event2('b',String::from("abc"),10);
    e4=en::event3{a:String::from("def"),b:'d'};

    match e1
    {
        en::key=>
        {
            print!("key\n");
        },
        en::event1(ch)=>
        {
            print!("event1\n");
        },

        en::event2(ch1,str1,int1)=>
        {
            print!("event2\n");
        }
        en::event3{a,b}=>
        {
            print!("event3\n");
        },
    }

}