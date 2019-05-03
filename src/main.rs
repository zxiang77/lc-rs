mod utils;

mod sol;
fn main() {
    let mut v = vec![];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{}", v[0]);
    println!("{}", v[1]);
    println!("{}", v[2]);

    v.pop();
    println!("{}", v[0]);
}

