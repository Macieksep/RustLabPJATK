// Zestaw zadań: 9 (początkujący)
// Autor: Maciej Sepeta s30518
// Wykonane w ramach koła naukowego Rust Lab PJATK

use std::rc::Rc;

fn main() {

    zad1();

    zad2();

    zad3();

}

fn zad1() {

    let pudlo :Box<i32> = Box::new(10);

    println!("{}", *pudlo)

}

fn zad2() {

    let rc :Rc<String> = Rc::new(String::from("Hello"));

    let ref1 = Rc::clone(&rc);
    let ref2 = Rc::clone(&rc);

    println!("{}", Rc::strong_count(&rc))

}

fn zad3() {

    let rc :Rc<Vec<i32>> = Rc::new(vec![1,2,3,4,5]);

    let ref1 = Rc::clone(&rc);

    for &num in ref1.iter() {
        println!("{}", num)
    }

}