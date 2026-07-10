#[derive(Debug)]
enum ipaddrkind{
    V4(u8,u8,u8),
    V6(String),
}


fn main() {
   let home=ipaddrkind::V4(6,7,8,);
   let loopback=ipaddrkind::V6(String::from("093ensok"));
    dbg!(&home);
    dbg!(&loopback);
    println!("{home:#?}");

}
