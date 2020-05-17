use std::io;
fn main() {
    println!("This is a BUBBLE SHORT");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let array_size = input.trim().parse().expect("Please write correct form");
    let mut array  = [0;4];
    for i in 0..array_size {
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        let number = num.trim().parse::<i32>().expect("Please enter a number");
        array[i] = number;
    }
    println!("Here is your array {:?}",array);
    let mut swap_count=0;
    for i in 0..array_size{
        for j in 0..array_size-1{
            if array[j] > array[j+1]{
            let temp  = array[j+1];
            array[j+1] = array[j];
            array[j] = temp;
            swap_count+=1;
            }
        }
    }
    println!("Buubled Short array : {:?} number of swap {}",array,swap_count);
}
