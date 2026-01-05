#![cfg(test)]
use std::time::{Duration, Instant};

use crate::batches::*;
// use crate::triples_array::*;

#[inline(always)]
pub fn print_elapsed(start: Instant) {
    let duration = start.elapsed();
    println!("elapsed: {}", format_duration(duration));
}

pub fn format_duration(duration: Duration) -> String {
    format!("{}.{:0>3}s", duration.as_secs(), duration.as_millis() % 1000)
}

pub fn log_total_and_mean(duration: Duration, count: f64) {
    println!("total: {}", format_duration(duration));
    println!("mean: {}", format_duration(duration.div_f64(count)));
}

const SINGLETON_MIN: u32 = 10;
const INCLUDE_PRE: bool = false;
const SPLIT: bool = false;

#[test]
pub fn phi_eq_omicron_singleton() {
    let min = SINGLETON_MIN*SINGLETON_MIN.isqrt();
    let max = 10000;
    if INCLUDE_PRE {
        phi_eq_omicron(2, min, true);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_eq_omicron(min+1, mid, true);
        phi_eq_omicron(mid+1, max, true);
    } else {
        phi_eq_omicron(min, max, true);
    }
}

fn phi_eq_omicron(min: u32, max: u32, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::phi_equals_omicron(phi, 0);
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_x_omicron_singleton() {
    let max = 27;
    if INCLUDE_PRE {
        phi_x_omicron(2, SINGLETON_MIN, true);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_x_omicron(SINGLETON_MIN+1, mid, true);
        phi_x_omicron(mid+1, max, true);
    } else {
        phi_x_omicron(SINGLETON_MIN, max, true);
    }
}
    // // println!("2..{SINGLETON_MIN}");
    // phi_x_omicron(2, SINGLETON_MIN, true);
    // // println!("{}..{mid}",SINGLETON_MIN+1);
    // phi_x_omicron(SINGLETON_MIN+1, mid, true);
    // // println!("{}..{max}", mid+1);
    // phi_x_omicron(mid+1, max, true);

fn phi_x_omicron(min: u32, max: u32, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::phi_equals_omicron(phi, 0).phi_x_omicron().phi_x_omicron();
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_2_n_phi_p_1_singleton() {
    let max = 60;
    if INCLUDE_PRE {
        phi_2_n_phi_p_1(2, SINGLETON_MIN, true);
    }
    if SPLIT {
        let mid: u32 = (max+SINGLETON_MIN)/2;
        phi_2_n_phi_p_1(SINGLETON_MIN+1, mid, true);
        phi_2_n_phi_p_1(mid+1, max, true);
    } else {
        phi_2_n_phi_p_1(SINGLETON_MIN, max, true);
    }
}

fn phi_2_n_phi_p_1(min: u32, max: u32, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::phi_2_n_phi_p_1(phi, 0);
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_2_singleton() {
    let max = 60;
    if INCLUDE_PRE {
        phi_2(2, SINGLETON_MIN, true);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_2(SINGLETON_MIN+1, mid, true);
        phi_2(mid+1, max, true);
    } else {
        phi_2(SINGLETON_MIN, max, true);
    }
}

fn phi_2(min: u32, max: u32, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::phi_2(phi, 0);
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn par_phi_2_comp_singleton() {
    let max = 60;
    if INCLUDE_PRE {
        par_phi_2_comp(2, SINGLETON_MIN, true);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        par_phi_2_comp(SINGLETON_MIN+1, mid, true);
        par_phi_2_comp(mid+1, max, true);
    } else {
        par_phi_2_comp(SINGLETON_MIN, max, true);
    }
}

fn par_phi_2_comp(min: u32, max: u32, log: bool) -> (Duration, Duration) {
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::phi_2(phi, 0);
    }
    let seq_elapsed = start.elapsed();
    println!("seq");
    if log {
        log_total_and_mean(seq_elapsed, (max - min + 1) as f64);
    }
    
    let start = Instant::now();
    for phi in min..=max {
        #[expect(unused_must_use)]
        Batches::par_phi_2(phi, 0);
    }
    let par_elapsed = start.elapsed();
    println!("par");
    if log {
        log_total_and_mean(par_elapsed, (max - min + 1) as f64);
    }

    (seq_elapsed, par_elapsed)
}

#[test]
pub fn batches_building() {
    println!("phi == omicron");
    phi_eq_omicron_singleton();
    
    println!();
    println!("phi x omicron");
    phi_x_omicron_singleton();  

    println!();
    println!("phi^2 - phi + 1");
    phi_2_n_phi_p_1_singleton();

    println!();
    println!("phi^2");
    phi_2_singleton();
}