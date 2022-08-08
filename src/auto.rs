#![warn(non_snake_case)]
pub use enigo::*;
pub use fltk::button;
use fltk::button::Button;
use fltk::input::IntInput;
pub use fltk::prelude::*;
pub use std::cell::Cell;
pub use std::fs::{read_to_string, File};
use std::io::Read;
pub use std::sync::{Arc, Mutex};
pub use std::thread as thr;
pub use std::thread::sleep;
pub use std::time::Duration;
pub fn auto_clicker(
    num: Arc<Mutex<u64>>,
    stop: Arc<Mutex<bool>>,
    repeat_amount: Arc<Mutex<u64>>,
) -> Button {
    let mut b = button::Button::default()
        .with_size(60, 40)
        .with_label("START")
        .with_pos(140, 200);
    let stop_clone = Arc::clone(&stop);
    let s = stop_clone.lock().unwrap();
    drop(s);
    b.set_callback(move |_c|/*When the button is pressed */ {
        let a = Arc::clone(&stop);
        let rep = Arc::clone(&repeat_amount);
        let mut a = a.lock().unwrap();
        *a = false;
        drop(a);
        let a = Arc::clone(&stop);
        let num = Arc::clone(&num);
        let _thread1 = thr::spawn(move || {
            thr::sleep(Duration::from_secs(5));
            let repeat_num = rep.lock().unwrap();
            let delay = num.lock().unwrap();
            let mut e = Enigo::new();
            e.mouse_move_to(500, 100); // move the mouse away from the button
            for _ in 1..=*repeat_num {
                let check = a.lock().unwrap();
                if *check == true {
                    println!("breaking {}", *check);
                    break;
                }
                e.mouse_click(MouseButton::Left);
                println!("clicked {}", *check);
                drop(check);
                thr::sleep(Duration::from_millis(*delay));
            }
            println!("Ended");
        });
    });
    b
}
pub fn exit(stop: Arc<Mutex<bool>>) {
    let mut but = button::Button::default()
        .with_pos(80, 200)
        .with_size(60, 40)
        .with_label("EXIT");
    but.set_callback(move |_c| {
        let s = Arc::clone(&stop);
        println!("pressed stop");
        let _x = thr::spawn(move || {
            println!("started thread");
            let mut enigo = Enigo::new();
            enigo.mouse_move_to(500, 100);
            let mut s = s.lock().unwrap();
            *s = true;
        });
    });
}
pub fn read_write_box(some_input: &mut IntInput, some_arc: Arc<Mutex<u64>>) {
    let mut input_delay = some_arc.lock().unwrap();
    let d = some_input.value(); // get the value of the input box when clicked
    let d = d.replace("-", ""); // makes sure the number is not negative
    println!("string :{}", d); // test
    let d: u64 = d.parse().unwrap();
    *input_delay = d;
    println!(" ARC :{}", *input_delay); // test
}

pub fn key_presser<'a>(
    path: Arc<Mutex<String>>,
    mut button1: Button,
    stop: Arc<Mutex<bool>>,
    repeat_amount: Arc<Mutex<u64>>,
) {
    let mut b = Button::default()
        .with_size(60, 40)
        .with_label("RUN MACRO")
        .with_pos(110, 260);
    b.set_callback(move |_| {
        let s = Arc::clone(&stop);
        let mut s = s.lock().unwrap(); // makes the stop is false so It can run
        *s = false;
        drop(s);
        //button1.deactivate();
        let p = Arc::clone(&path);
        let stop1 = Arc::clone(&stop);
        let stop2 = Arc::clone(&stop);
        let rep = Arc::clone(&repeat_amount);
        thr::spawn(move || {
            let mut contents = String::new(); // will store the data of the file
            let p = p.lock().unwrap();
            let a = &*p;
            let mut f = File::open(a).expect("Unable to find the file");
            f.read_to_string(&mut contents)
                .expect("Unable to read the file");
            let mut enigo = Enigo::new();
            let rep = rep.lock().unwrap();
            for _ in 1..=*rep {
                let s = stop2.lock().unwrap();
                if *s == true {
                    drop(s);
                    println!("Ending");
                    break;
                }
                drop(s);
                for control in contents.split("/") {
                    let s = stop1.lock().unwrap();
                    if *s == true {
                        drop(s);
                        println!("Ending macro");
                        break;
                    }
                    match control {
                        "<click>" => {
                            enigo.mouse_click(MouseButton::Left);
                        }
                        "<Rclick>" => {
                            enigo.mouse_click(MouseButton::Right);
                        }
                        control if control.contains("|") => {
                            let c: char = control.replace("|", "").parse().unwrap();
                            enigo.key_down(Key::Layout(c));
                            thr::sleep(Duration::from_millis(1100));
                            enigo.key_up(Key::Layout(c));
                        }
                        _ => {
                            let c: char = control.parse().unwrap();
                            enigo.key_click(Key::Layout('h'));
                        }
                    }
                    enigo.key_click(Key::Layout('s'));
                    thr::sleep(Duration::from_millis(1000));
                }
            }
        });
    });
}
