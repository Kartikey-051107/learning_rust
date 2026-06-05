struct Rect{
    width:u32,
    hieght:u32,

}

fn area(rect:&Rect)->u32{
rect.width*rect.hieght
}
fn main(){
    let rectangle=Rect{
        width:30,
        hieght:50,
    };
    println!{"area of rectangle is {}",area(&rectangle)};
    

}