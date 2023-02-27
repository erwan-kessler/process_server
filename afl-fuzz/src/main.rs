#![allow(dead_code)]
#[macro_use]
extern crate afl;

fn main() {
    fuzz1();
}

fn fuzz1() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
        }
    });
}
