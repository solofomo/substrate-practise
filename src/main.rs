use substratepractise::*;

fn main() {
    let mut nums = [3, 2, 7, 10, 5];
    println!("input {:?}", nums);
    bubble_sort(&mut nums);
    println!("bubble sort for int {:?}", nums);
    

    let mut nums = [9.5, 2.4, 3.0, 18.1, 6.3];
    println!("input {:?}", nums);
    bubble_sort_generic(&mut nums); 
    println!("bubble sort for generic {:?}", nums);
}