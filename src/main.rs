// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2022 Andrew Powers-Holmes <aholmes@omnom.net>
//
// Simple little rust program to do some cute ASCII mandelbrot stuff.
// Created to test compilation and execution of floating-point Rust code
// in the OpenWrt build environment.

#![forbid(unsafe_code)]

use crossterm::terminal;
use num::complex::Complex;
use shadow_rs::shadow;
use std::cmp;

// gather build info
shadow!(build);

// configure floating-point precision based on CPU features
#[cfg(feature = "f32")]
pub type Float = f32;
#[cfg(feature = "f32")]
const PRECISION: &str = "single";
#[cfg(not(feature = "f32"))]
pub type Float = f64;
#[cfg(not(feature = "f32"))]
const PRECISION: &str = "double";

// flexible-precision complex number type
pub type FlexComplex = Complex<Float>;

// configure max iterations based on CPU features
#[cfg(feature = "u64")]
pub type Iter = u64;
#[cfg(not(feature = "u64"))]
pub type Iter = u32;

// functions to calculate the mandelbrot set for a given point
struct Ifs {
    max_iter: Iter,
}

trait Dds<State> {
    fn cont(&self, z: State) -> bool;
    fn next(&self, z: State, c: State) -> State;
}

impl Dds<FlexComplex> for Ifs {
    fn cont(&self, z: FlexComplex) -> bool {
        z.norm_sqr() <= 4.0
    }

    fn next(&self, z: FlexComplex, c: FlexComplex) -> FlexComplex {
        z * z + c
    }
}

impl Ifs {
    pub fn new(max_iter: Iter) -> Self {
        Self { max_iter }
    }

    pub fn iter(&self, c: FlexComplex) -> Iter {
        let mut i: Iter = 0;
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

// changes an intensity into an ascii character
fn val_to_char(value: u8) -> char {
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

// main execution
fn main() {
    // work out what size terminal we have to work with
    let termsize: (u16, u16) = terminal::size().unwrap_or((80, 25));

    // clamp minimum and maximum dimensions to something reasonable
    let cols = cmp::min(cmp::max(termsize.0 as usize, 80), 128);
    let rows = cmp::min(cmp::max(termsize.1 as usize, 40), 128);

    // print some info about what we're doing
    println!(
        "float_test v{} {} for {} ({} precision)",
        build::PKG_VERSION,
        build::BUILD_RUST_CHANNEL,
        build::BUILD_TARGET,
        PRECISION
    );
    println!(
        "built with {} at {} on a {} host",
        build::RUST_VERSION,
        build::BUILD_TIME_2822,
        build::BUILD_OS,
    );
    println!(
        "{}x{} terminal, will output {}x{} characters",
        termsize.0, termsize.1, cols, rows
    );

    // do math for and render mandelbrot set
    let min = Complex::new(-1.4, -1.0);
    let max = Complex::new(0.6, 1.0);
    let mandel = Ifs::new(256);

    for row in 0..rows {
        for col in 0..cols {
            let x = min.re + (max.re - min.re) * (col as Float) / (cols as Float);
            let y = min.im + (max.im - min.im) * (row as Float) / (rows as Float);
            let c = Complex::new(x, y);
            let m = mandel.iter(c) as u8;
            print!("{}", val_to_char(m));
        }
        println!();
    }
}
