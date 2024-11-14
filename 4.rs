use std::fs::File;
use std::io;
use std::io::Read;

// Zestaw zadań: 4 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3()

}

fn zad1() {

    println!("{:?}", divide_numbers(2.0, 5.0));
    println!("{:?}", divide_numbers(10.0, 0.0));

}

fn divide_numbers(num :f32, div :f32) -> Result<f32, String> {

    match div {
        0.0 => Err(String::from("Nie dziel przez 0!!!")),
        _ => Ok(num/div)
    }

}

fn zad2() {
    println!("{}", parse_integer(String::from("123456")));
    println!("{}", parse_integer(String::from("1x2x3x4x5x6")));
    println!("{}", parse_integer2(String::from("1x2x3x4x5x6")));
}

fn parse_integer(str :String) -> i32{

    str.parse::<i32>()
        .expect("Błąd ciągu znaków!!!")

}

fn parse_integer2(str :String) -> i32{

    str.parse::<i32>()
        .unwrap()

}

fn zad3() {

    println!("{:?}", read_file("test.txt"));

}

fn read_file(file_path :&str) -> Result<String, io::Error>{

    let mut file = File::open(file_path)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)

}