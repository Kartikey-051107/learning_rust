fn main(){

    let   mut  s1=String::from("i am here ");
    s1.push_str("yahoo");
    let len=calculate_len(&mut s1);
    println!("values are {s1} and {len}");



}
fn calculate_len(s:& mut String)->usize{

    let result=s.len();
    result



}
