use std::fs::File;
use std::io::{Read, Write};

// Zestaw zadań: 2 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3();

    zad4();

}

// ZAD 1
fn zad1() {
    let book = Book { title: String::from("Hobbit"), author: String::from("J.R.R. Tolkien") };
    book.describe();
}

struct Book {

    title: String,
    author: String

}

impl Book {

    fn describe(&self) {
        println!("Tytuł: {}, Autor: {}", self.title, self.author)
    }

}

// ZAD 2

fn zad2() {

    let vec = vec![1, 2, 3, 4, 5];
    let result = find_element(&vec, 2);
    println!("{:?}", result);

}

fn find_element(vec: &Vec<i32>, num :i32) -> Option<usize> {
    for i in 0..vec.len() {
        if let Some(&current) = vec.get(i) {
            if current == num {

                return Some(i);

            }
        }
    }

    None

}

// ZAD 3

fn zad3() {

    let result = divide(10, 2);
    println!("{:?}", result); // Wynik: Ok(5.0)

}

fn divide(a :i32, b :i32) -> Result<f64, String> {

    if b != 0 {
        Ok((a/b) as f64)
    } else {
        Err(String::from("Nie dziel przez 0 -_-"))
    }

}

// ZAD 4

fn zad4() {

    write_to_file("output.txt", "Witaj, Rust!");
    read_from_file("output.txt");

}

fn write_to_file(file_path:&str, text :&str) {

    let mut file = File::create(file_path);

    file.unwrap().write_all(text.as_ref()).expect("Blad!");

}

fn read_from_file(file_path :&str) {

    let mut file = File::open(file_path);

    let mut content = String::new();

    file.unwrap().read_to_string(&mut content).expect("Blad!");

    println!("{}", content);

}