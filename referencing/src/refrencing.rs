fn main() {
    let   s1=String::from("i am here ");
    let len=calculate_len(&s1);
    println!("values are {s1} and {len}");



}
fn calculate_len(s:&String)->usize{

    let result=s.len();
    result



}