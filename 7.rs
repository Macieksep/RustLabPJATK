// Zestaw zadań: 7 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

use std::io::Read;
use std::path::Path;

fn main() {

    //UWAGA: Zamiast argumentów użyłem zwykłego wejścia, trochę z lenistwa ;)

    zad1();

    zad2();

    zad3();

}

fn zad1() {

    let mut sciezka = String::new();

    std::io::stdin().read_line(&mut sciezka).expect("blad");

    let sciezka = sciezka.trim_end();

    println!("{}", std::fs::read_to_string(Path::new(&sciezka)).expect("nie znaleziono pliku"));

}

fn zad2() {

    let mut sciezka = String::new();

    std::io::stdin().read_line(&mut sciezka).expect("blad");

    let sciezka = sciezka.trim_end();

    let mut tekst = String::new();

    std::io::stdin().read_line(&mut tekst).expect("blad");

    std::fs::write(Path::new(sciezka), tekst).expect("blad zapisu");

}

fn zad3() {

    std::fs::read_to_string(Path::new("nie/istnieje.txt"))
        .unwrap_or_else(|err| {
            String::from("Nie dziala X.X")
        });

    std::fs::write(Path::new("nie/istnieje.txt"), "xyz")
        .unwrap_or_else(|err| {
            println!("Nie dziala X.X")
        });

}