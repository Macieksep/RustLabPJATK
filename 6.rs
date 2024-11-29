// Zestaw zadań: 6 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {
    // :)
}

#[cfg(test)]
mod zad1 {

    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(1, 2), 3);
    }

}

fn add(a: i32, b: i32) -> i32 {

    a + b

}

#[cfg(test)]
mod zad2 {

    use super::*;

    #[test]
    fn is_positive_test() {
        assert!(is_positive(5));
    }

}

fn is_positive(a: i32) -> bool {

    a > 0 // zakładając, że 0 jest nijakie

}

#[cfg(test)]
mod zad3 {

    use super::*;

    #[test]
    fn divide_test_1() {
        assert_eq!(divide(10, 5), Some(2));
    }

    #[test]
    fn divide_test_2() {
        assert_eq!(divide(10, 0), None);
    }

}

fn divide(a: i32, b: i32) -> Option<i32> {

    if b == 0 {
        None
    } else {
        Some(a/b)
    }

}