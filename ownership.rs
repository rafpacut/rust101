fn main() {
    let s = String::from("abc");
    takes_ownership(s);
    println!("{}",s);
}

fn takes_ownership(s: String)
{
    println!("in function: {}",s);
}
