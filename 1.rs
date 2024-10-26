use std::io;
use std::io::Write;
use rand::Rng;

// Zestaw zadań: 1 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3();

}

// ZADANIE 1

fn zad1() {

    let mut input_line= String::new();
    let mut a :f32;
    let mut b :f32;
    let mut op;

    print!("Podaj pierwszą liczbę: ");
    io::stdout().flush().expect("");
    io::stdin().read_line(&mut input_line)
        .expect("Błąd!");
    a = input_line.trim().parse().expect("Nie float!");
    input_line.clear();

    print!("Podaj drugą liczbę: ");
    io::stdout().flush().expect("");
    io::stdin().read_line(&mut input_line)
        .expect("Błąd!");
    b = input_line.trim().parse().expect("Nie float!");
    input_line.clear();

    print!("Wybierz operację (+, -, *, /): ");
    io::stdout().flush().expect("");
    io::stdin().read_line(&mut input_line)
        .expect("Błąd!");
    op = input_line.trim().parse().expect("Nie char!");

    println!("Wynik: {}", operacja(a, b, op))
}

fn operacja(a :f32, b :f32, op :char) -> f32{

    match op {
        '+' => dodawanie(a,b),
        '-' => odejmowanie(a, b),
        '*' => mnozenie(a, b),
        '/' => dzielenie(a, b),
        _   => panic!("Błędna operacja!")
    }

}

fn dodawanie(a :f32, b :f32) -> f32 {a+b}
fn odejmowanie(a :f32, b :f32) -> f32 {a-b}
fn mnozenie(a :f32, b :f32) -> f32 {a*b}
fn dzielenie(a :f32, b :f32) -> f32 {a/b}

// ZADANIE 2

fn zad2() {

    let mut input_line = String::new();
    let mut temp :f32;
    let mut jedn :char;

    print!("Podaj temperaturę: ");
    io::stdout().flush().expect("");
    io::stdin().read_line(&mut input_line)
        .expect("Błąd!");
    temp = input_line.trim().parse().expect("Nie float!");
    input_line.clear();

    print!("Konwertować na (F)ahrenheita czy (C)elsjusza? ");
    io::stdout().flush().expect("");
    io::stdin().read_line(&mut input_line)
        .expect("Błąd!");
    jedn = input_line.trim().parse().expect("Nie char!");
    input_line.clear();

    print!("Wynik: {}", konwerter(temp, jedn));

    if jedn == 'f' || jedn == 'F' {

        println!(" C")

    } else {

        println!(" F")

    }

}

fn konwerter(temp :f32, jedn :char) -> f32 {

    match jedn {
        'f'|'F' => (temp * 1.8) + 32f32,
        'c'|'C' => (temp - 32f32) / 1.8,
        _ => panic!("Błędna jednostka!"),
    }

}

// ZADANIE 3

fn zad3() {

    let rand_num = rand::thread_rng().gen_range(1..11);
    let mut proby = 3;
    let mut input_line = String::new();
    let mut proba :i32;

    while proby > 0 {

        print!("Pozostało {} prób. Podaj liczbę (1-10): ", proby);
        io::stdout().flush().expect("");
        io::stdin().read_line(&mut input_line)
            .expect("Błąd!");
        proba = input_line.trim().parse().expect("Nie integer!");
        input_line.clear();

        if rand_num == proba {
            println!("Trafiony zatopiony! Liczba to: {}", proba);
            break;
        } else {
            if proby == 1 {
                println!("Pódło! Przegrałeś EZ");
            } else {
                println!("Pódło! Spróbuj jeszcze raz :)");
            }

            proby -= 1;

        }

    }
}