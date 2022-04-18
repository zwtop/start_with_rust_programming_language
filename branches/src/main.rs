fn main() {
    let mut x = if 5 > 0 { 5 } else { 0 };
    println!("This value of x is {}", x);

    let y = 'lp: loop {
        loop {
            if x % 4 == 0 {
                break 'lp x;
            }
            x = x + 1;
            continue 'lp;
        }
    };
    println!("The value of y is {}", y);

    while x > 0 {
        println!("The value of x is {}", x);
        x -= 1;
    }

    for number in (1..5).rev() {
        println!("The number is {}", number);
    }
}
