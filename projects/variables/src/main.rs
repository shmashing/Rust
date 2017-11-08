fn main() {
    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("x: {}",x);
    
    let tup = (5,10,15);
    let (i,j,k) = tup;
    println!("Middle element of tuple: {}",j);

    let arr = [0,1,2,3,4];
    println!("Second element of array: {}",arr[1]);
}
