// Zestaw zadań: 5 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3();

}

fn zad1() {

    println!("Wieksza wartosc: {}", compare_values(1, 2));
    println!("Wieksza wartosc: {}", compare_values(3.5, 2.1));
    println!("Wieksza wartosc: {}", compare_values('a', 'z'));
    println!("Wieksza wartosc: {}", compare_values("BBB", "AAA"));

}

fn compare_values<T: PartialOrd>(a: T, b: T) -> T {

    if a > b {
        a
    } else {
        b
    }

}

fn zad2() {

    let p :Person = Person {
        name: String::from("Bob"),
        age: 420
    };

    p.print_info()

}

struct Person {
    name: String,
    age: u32
}

trait Printable {
    fn print_info(&self);
}

impl Printable for Person {
    fn print_info(&self) {
        println!("{}, {}", self.name, self.age)
    }
}

fn zad3() {

    let a : &str = "abc";
    let b : &str = "abcdef";
    let c : &str = "ab";

    println!("{}", longest_str(a, b));
    println!("{}", longest_str(b, c));

}

fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {

    if a.len() > b.len() {
        a
    } else {
        b
    }

}