//! This module contains the tiling functionality.

extern crate lazy_static;

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::{thread, time};

//todo

lazy_static!{
    static ref OWO: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
}

pub async fn s1() {
    loop {
        OWO.lock().unwrap().push(69);
        println!("{:?}", OWO.lock().unwrap());
        thread::sleep(time::Duration::from_millis(750));
    }
}

pub async fn s2() {
    loop {
        println!("{:?}", OWO.lock().unwrap());
        thread::sleep(time::Duration::from_millis(750));
    }
}
