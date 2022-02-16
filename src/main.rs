// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2022 Andrew Powers-Holmes <aholmes@omnom.net>
//
// Simple little rust program to do some cute ASCII mandelbrot stuff.
// Created to test compilation and execution of floating-point Rust code
// in the OpenWrt build environment.

use crossterm::terminal;
use num::complex::Complex;
use std::cmp;
use std::env::consts::{ARCH, OS};

struct Ifs {
    max_iter: u64,
}

trait Dds<State> {
    fn cont(&self, z: State) -> bool;
    fn next(&self, z: State, c: State) -> State;
}

impl Dds<Complex<f64>> for Ifs {
    fn cont(&self, z: Complex<f64>) -> bool {
        z.norm_sqr() <= 4.0
    }

    fn next(&self, z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
        z * z + c
    }
}

impl Ifs {
    pub fn new(max_iter: u64) -> Self {
        Self { max_iter }
    }

    pub fn iter(&self, c: Complex<f64>) -> u64 {
        let mut i: u64 = 0;
        let mut z = c;
        while i < self.max_iter && self.cont(z) {
            z = self.next(z, c);
            i += 1;
        }
        if i < self.max_iter {
            return self.max_iter - i;
        }
        0
    }
}

fn val_to_char(value: u8) -> char {
    // changes an intensity into an ascii character
    let chars = ['@', '%', '#', '*', '+', '=', '~', ':', '.', ' '];

    let num_chars: u8 = chars.len() as u8;
    let step: u8 = (255 / num_chars) as u8;

    for i in 0..(num_chars - 1) {
        if value >= i * step && value < (i + 1) * step {
            return chars[i as usize];
        }
    }
    chars[(num_chars - 1) as usize]
}

fn main() {
    let termsize: (u16, u16) = terminal::size().unwrap_or((80, 25));

    let cols = cmp::min(cmp::max(termsize.0 as usize, 64), 128);
    let rows = cmp::min(cmp::max(termsize.1 as usize, 64), 128);
    let aspect = rows as f64 / cols as f64;

    println!(
        "running on {} {} with {}x{} terminal, aspect ratio {}",
        OS, ARCH, termsize.0, termsize.1, aspect
    );

    let min = Complex::new(-1.4, -1.0);
    let max = Complex::new(0.6, 1.0);
    let mandel = Ifs::new(256);

    for row in 0..rows {
        for col in 0..cols {
            let x = min.re + (max.re - min.re) * (col as f64) / (cols as f64);
            let y = min.im + (max.im - min.im) * (row as f64) / (rows as f64);
            let c = Complex::new(x, y);
            let m = mandel.iter(c) as u8;
            print!("{}", val_to_char(m));
        }
        println!();
    }
}
