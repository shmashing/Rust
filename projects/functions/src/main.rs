fn main() {
    let x = five();
    println!("the value of x: {}",x);

    let x = add_one(five());
    println!("the value of x: {}",x);

    let x = hello_world();
    println!("the value of x: {}",x);

    let mut array = Vec::new();
    array.push(1);
    array.push(2);
    array.push(3);
    array = change_array(array);
    println!("{:?}", array);
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
fn change_array(mut arr: Vec<u32>) -> Vec<u32>{
    arr[0] = 5;

    arr
}
