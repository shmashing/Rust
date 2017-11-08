fn main() {
    let x = five();
    println!("the value of x: {}",x);

    let x = add_one(five());
    println!("the value of x: {}",x);

    let x = hello_world();
    println!("the value of x: {}",x);
}
fn five() -> i32 {
    5
}
fn add_one(x: i32) -> i32 {
    x+1
}
fn hello_world() -> &'static str {
    println!("in hello world funct");
    "Hello World"
}
