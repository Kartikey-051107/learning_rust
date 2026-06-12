struct Rectangle{
    width:u32,
    hieght:u32,
}

fn main(){
    impl Rectangle{
        fn square(size:u32)->Self{
            Self{
                width:size,
                hieght:size,

            }
        }
        
    }
    let sq=Rectangle::square(5);

}