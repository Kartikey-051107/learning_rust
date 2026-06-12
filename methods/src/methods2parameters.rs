struct Rectangle{
    width:u32,
    hieght:u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        hieght: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        hieght: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        hieght: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.hieght
    }
    fn can_hold(&self,other:&Rectangle)->bool{
    self.width>other.width && self.hieght>other.hieght
    }
}