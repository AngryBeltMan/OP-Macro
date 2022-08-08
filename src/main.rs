// use enigo::*;
use fltk::button::Button;
use fltk::input::{Input, IntInput};
use fltk::prelude::*;
use fltk::{app, window};
use std::sync::{Arc, Mutex};
mod auto;
fn main() {
    // all of the Arcs
    let stop = Arc::new(Mutex::new(false));
    let cps_speed = Arc::new(Mutex::new(1000u64));
    let loop_arc = Arc::new(Mutex::new(100u64));
    let path = Arc::new(Mutex::new(String::from("")));
    // - - - - - - - - --
    let app = app::App::default();
    let mut window = window::Window::default() // makes the gui window
        .with_size(300, 300)
        .with_label("OP MACRO");
    app::background(211, 211, 211);
    let speed = IntInput::default()
        .with_label("DELAY (ms):")
        .with_size(90, 35)
        .with_pos(100, 10);
    let repeat_amount = IntInput::default()
        .with_label("Repeat (num):")
        .with_size(90, 35)
        .with_pos(100, 50);
    let path_input = Input::default()
        .with_label("Path (txt file):")
        .with_size(90, 35)
        .with_pos(100, 90);
    speed_input(speed, Arc::clone(&cps_speed)); // makes an input box asking delay per click
    loop_amount(repeat_amount, Arc::clone(&loop_arc));
    path_location(path_input, Arc::clone(&path));
    let mut start_button = auto::auto_clicker(
        Arc::clone(&cps_speed),
        Arc::clone(&stop),
        Arc::clone(&loop_arc),
    );
    auto::exit(Arc::clone(&stop));
    auto::key_presser(
        Arc::clone(&path),
        start_button,
        Arc::clone(&stop),
        Arc::clone(&loop_arc),
    );
    window.end();
    window.show();
    app.run().unwrap();
}
fn speed_input(mut input: IntInput, delay: Arc<Mutex<u64>>) /* ask for delay */
{
    let mut b = Button::default()
        .with_pos(190, 10)
        .with_size(40, 35)
        .with_label("submit");
    b.set_callback(move |_| {
        auto::read_write_box(&mut input, Arc::clone(&delay));
    });
    // go to auto.rs line 62 for more info
}

fn loop_amount(mut input: IntInput, repeat_amount: Arc<Mutex<u64>>)
/* Ask for how much times to loop */
{
    let mut b = Button::default()
        .with_pos(190, 50)
        .with_size(40, 35)
        .with_label("submit");
    b.set_callback(move |_| {
        auto::read_write_box(&mut input, Arc::clone(&repeat_amount));
    });
}
fn path_location(mut input: Input, path: Arc<Mutex<String>>) {
    let mut b = Button::default()
        .with_pos(190, 90)
        .with_size(40, 35)
        .with_label("submit");
    b.set_callback(move |_| {
        let mut p = path.lock().unwrap();
        let text_file_loc = input.value();
        *p = text_file_loc;
        println!("{}", *p);
    });
}
