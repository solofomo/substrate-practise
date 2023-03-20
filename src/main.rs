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

    let red = TrafficLight::Red(5);
    let green = TrafficLight::Green(10);
    let yellow = TrafficLight::Yellow(1);

    println!("Red light: {} seconds", red.duration());  // 输出 Red light: 5 seconds
    println!("Green light: {} seconds", green.duration());  // 输出 Green light: 10 seconds
    println!("Yellow light: {} seconds", yellow.duration());  // 输出 Yellow light: 1 seconds

    let numbers = [1, 2, 3, 4, 5];
    match sum_u32(&numbers) {
        Some(result) => println!("Sum: {}", result),
        None => println!("溢出"),
    }

    let circle = Circle { radius: 2.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 2.5 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}