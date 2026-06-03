struct Player{
    name:String,
    runs:u64,
    country:String,
    retired:bool,



}
fn main(){
    let p1=Player{
        name:String::from("Rohit Sharma"),
        runs:18657,
        country:String::from("India"),
        retired:false,

    };
    println!("{} is a player from {}.He has scored {} runs and his retirement status is {}",p1.name,p1.country,p1.runs
,p1.retired);
}