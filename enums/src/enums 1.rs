
#[derive(Debug)]
enum Ipaddrtype{
   
    V4,
    V6,
}
#[derive(Debug)]
struct IPaddr{
     
    kind:Ipaddrtype,
    addr:String,
    }

fn main() {
    let home =IPaddr{
        kind:Ipaddrtype::V6,
        addr:String::from("foheofb"),

    };
    let loopback =IPaddr{
        kind:Ipaddrtype::V4,
        addr:String::from("ieorgowein"),
    };
    dbg!(&home);
    dbg!(&loopback);
    
    


   

}
