use uncollate::{Uncollate};

#[derive(Uncollate)]
pub struct Basic {
    a: bool,
    b: u64,
}

fn main(){
    let basics = vec![
        Basic{ a: true, b: 12 },
        Basic{ a: false, b: 43 }
    ];
    let uc = basics.uncollate();

    println!("{:?}", uc.a());
    println!("{:?}", uc.b());
}