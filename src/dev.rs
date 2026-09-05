#![allow(unused)]

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use dialoguer::{Confirm, console};

#[test]
pub fn test_dep() {
    let mut message = "Lokasi tidak ditemukan".to_string();
    for _ in 0..4 {
        print!("\r{}", message);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(1000));
        message.push('.');
    }
    console::Term::stdout().clear_line().unwrap();

    println!("Restart Config");
}
