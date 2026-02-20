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
    let bs: &[Basic] = &basics;
    println!("{:?}", basics.uncoll_a());
    println!("{:?}", bs.uncoll_b());
}