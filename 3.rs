// Zestaw zadań: 3 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

fn main() {

    zad1();

    zad2();

    zad3()

}

fn zad1() {
    let str = String::from("Test 1");

    show_ownership(str);
}

fn show_ownership(str :String) {

    let new_str = str;

    //println!("{}", str) - błąd "borrow of moved value: `str`", nie kompiluje się

}

fn zad2() {

    let str = String::from("Test 2");

    show_borrowed(&str);
    show_borrowed(&str);
    show_borrowed(&str);
}

fn show_borrowed(str :&String) {

    println!("{}", str)

}

fn zad3() {

    let mut str = String::from("Test 3");

    append_exclamation(&mut str);

    println!("{}", str);

    let str_mut_test = &mut str;
    let str_mut_test2 = &mut str;

    // println!("{} {}", str_mut_test, str_mut_test2); - tu wyrzuca błąd "second mutable borrow occurs here" :)

}

fn append_exclamation(str :&mut String) {

    str.push_str("!");

}