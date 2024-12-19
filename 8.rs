// Zestaw zadań: 8 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3();

}

fn zad1() {

    let liczby = vec![1,2,3,4,5];

    println!("{}", sum_even_numbers(liczby))

}

fn sum_even_numbers(zbior_liczb: Vec<i32>) -> i32{

    let mut suma = 0;

    zbior_liczb
        .iter()
        .filter(|&&x| x % 2 == 0)
        .for_each(|x| suma += x);

    suma
}

fn zad2() {

    let liczby = vec![1,2,3,4,5];

    double_values(liczby)
        .iter()
        .for_each(|x| println!("{}", x))

}

fn double_values(zbior_liczb: Vec<i32>) -> Vec<i32>{

    zbior_liczb
        .iter()
        .map(|x| x * 2)
        .collect()

}

fn zad3() {

    let stringi = vec!["a", "bbb", "cccc"];

    get_strings_longer_than_three(stringi)
        .iter()
        .for_each(|x| println!("{}", x))

}

fn get_strings_longer_than_three(zbior_liczb: Vec<&str>) -> Vec<&str>{

    zbior_liczb
        .iter()
        .filter(|x| x.len() > 3)
        .cloned() // z &&str na &str
        .collect()

}