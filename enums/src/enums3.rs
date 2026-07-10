fn find_username(user_id:u32)->Option<String>{
    if user_id==42{
        Some(String::from("Alice"))
    }
    else{
        None
    }
}
fn main(){
    let user1=find_username(42);
    let user2=find_username(34);

    println!("{:?}",user1);
    println!("{:?}",user2);

}