
#[derive(Debug)]
struct Rectangle{
    width:u32,
    hieght:u32,

};
fn main(){
    let scale =2;
    let rect1=Rectangle{
        width :dbg!(30*scale),
        hieght:50,

    };
    dbg!(&rect1);

}
