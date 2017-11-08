fn main() {
    let x = 10;

    if x%5 == 0 {
        println!("{} divisible by 5",x);
    } else if x%4 == 0 {
        println!("{} divisible by 4",x);
    } else if x%3 != 0 {
        println!("{} NOT divisible by 3",x);
    } else if x%2 == 0 {
        println!("{} is even",x);
    }

    let condition = true;
    let x = if condition {
        5
    } else {
        6
    };
    println!("Value of x: {}", x);

    let mut x = 5;
    while x != 0 {
        println!("{}...",x);
        x = x-1;
    }
    println!("LIFT OFFFFFFF");

    for i in (1..5).rev(){
         println!("{}...",i);       
    }
    println!("LIFT OFFFFFFF");


    let a = [1,2,3,4,5];
    for element in a.iter() {
        println!("The value of the element: {}", element);
    }

    for number in 0..5 {
        println!("The value of element {}: {}", number, a[number]);
    }
}
