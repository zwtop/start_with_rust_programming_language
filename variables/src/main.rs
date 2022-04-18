use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter array index");

    // This is a comment for variable index
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed read line");

    let index: usize = index.trim().parse().expect("Index should be a number");

    println!("The index {} number is {}", index, a[index]);

    print_labeled_measurement(a[index], '元');

    expression_with_block();

    println!("Return five is {}", return_five());
}

fn print_labeled_measurement(value: u32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn expression_with_block() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y);
}

fn return_five() -> u8 {
    5
}
