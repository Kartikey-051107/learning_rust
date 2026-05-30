fn main() {
    let   s1=String::from("i am here ");
    let (s,len)=calculate_len(s1);
    println!("values are {s} and {len}");



}
fn calculate_len(s:String)->(String ,usize){

    let result=s.len();
    (s,result)



}