enum event
{
    event_1,
    event_2(String,String),
    event_3{v1:i16,v2:f32,v3:char},
}
fn main()
{

let case_1=event::event_1;
let case_2=event::event_2("abc".to_owned(),"def".to_owned());
let case_3=event::event_3{v1:10,v2:12.1,v3:'a'};/**/
matching(case_1);
matching(case_2);
matching(case_3);
}

fn matching(v:event)
{
    match v
    {
        event::event_1=>respond_1(),
        event::event_2(str1,str2)=>respond_2(),
        event::event_3{v1,v2,v3}=>respond_3(),
    }


}

fn respond_1()
{
    println!("respond_1 of event_1 ");
}
fn respond_2()
{
    println!("respond_2 of event_2 ");
}
fn respond_3()
{
    println!("respond_3 of event_3 ");
}

