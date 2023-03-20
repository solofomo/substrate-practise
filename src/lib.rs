pub fn bubble_sort(n: &mut [i32]) {
    let len = n.len();

    for i in 0..len {
        for j in 0..len-i-1 {
            if n[j] > n[j+1] {
                n.swap(j, j+1);
            }
        }
    }
}

pub fn bubble_sort_generic<T: PartialOrd>(n: &mut [T]) {
    let len = n.len();

    for i in 0..len {
        for j in 0..len-i-1 {
            if n[j] > n[j+1] {
                n.swap(j, j+1);
            }
        }
    }
}

pub enum TrafficLight {
    Red(u8),
    Yellow(u8),
    Green(u8),
}

pub trait TimeDuration {
    fn duration(&self) -> u8;
}

impl TimeDuration for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red(time) => *time,
            TrafficLight::Yellow(time) => *time,
            TrafficLight::Green(time) => *time,
        }
    }
}

pub fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;

    for &num in numbers.iter() {
        match sum.checked_add(num) {
            Some(new_sum) => sum = new_sum,
            None => return None,
        }
    }

    Some(sum)
}

pub fn print_area<T: Area>(shape: T) {
    println!("The area is {}", shape.area());
}

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

pub struct Square {
    pub side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut nums = [8, 7, 5, 6, 9];
        bubble_sort(&mut nums);
        assert_eq!(nums, [5, 6, 7, 8, 9]);
    }
    
    #[test]
    fn test_bubble_sort_generic() {
        let mut nums = [4.4, 2.2, 3.3, 1.1, 5.5];
        bubble_sort_generic(&mut nums);
        assert_eq!(nums, [1.1, 2.2, 3.3, 4.4, 5.5]);

        let mut strings = ["c", "a", "d", "b"];
        bubble_sort_generic(&mut strings);
        assert_eq!(strings, ["a", "b", "c", "d"]);
    }

    #[test]
    fn test_duration() {
        let red = TrafficLight::Red(30);
        let yellow = TrafficLight::Yellow(3);
        let green = TrafficLight::Green(20);

        assert_eq!(red.duration(), 30);
        assert_eq!(yellow.duration(), 3);
        assert_eq!(green.duration(), 20);
    }

    #[test]
    fn test_sum_u32() {
        let numbers = [1, 2, 3, 4, 5];
        assert_eq!(sum_u32(&numbers), Some(15));

        let numbers = [u32::MAX, 1];
        assert_eq!(sum_u32(&numbers), None);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 2.0 };
        assert_eq!(circle.area(), 12.566370614359172);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle { base: 3.0, height: 4.0 };
        assert_eq!(triangle.area(), 6.0);
    }

    #[test]
    fn test_square_area() {
        let square = Square { side: 2.5 };
        assert_eq!(square.area(), 6.25);
    }
}
