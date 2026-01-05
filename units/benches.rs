#![cfg(test)]
use std::hint::black_box;
#[allow(unused_imports)]
use std::io::{Write, stdout};
use std::time::{Duration, Instant};

use crate::Int;
#[allow(unused_imports)]
use crate::tests;
use crate::batches::*;
use crate::statics::*;
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

const SINGLETON_MIN: Int = 50;
const INCLUDE_PRE: bool = false;
const SPLIT: bool = false;
const SINGLETON_LOGS: bool = false;
#[test]
pub fn phi_eq_omicron_singleton() {
    let min = SINGLETON_MIN*SINGLETON_MIN.isqrt();
    let max = 10000;
    if INCLUDE_PRE {
        phi_eq_omicron(2, min, SINGLETON_LOGS);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_eq_omicron(min+1, mid, SINGLETON_LOGS);
        phi_eq_omicron(mid+1, max, SINGLETON_LOGS);
    } else {
        phi_eq_omicron(min, max, SINGLETON_LOGS);
    }
}

fn phi_eq_omicron(min: Int, max: Int, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
       black_box( Batches::phi_equals_omicron(phi, 0));
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_x_omicron_singleton() {
    let max = SINGLETON_MIN;
    if INCLUDE_PRE {
        phi_x_omicron(2, SINGLETON_MIN, SINGLETON_LOGS);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_x_omicron(SINGLETON_MIN+1, mid, SINGLETON_LOGS);
        phi_x_omicron(mid+1, max, SINGLETON_LOGS);
    } else {
        phi_x_omicron(SINGLETON_MIN, max, SINGLETON_LOGS);
    }
}
    // // println!("2..{SINGLETON_MIN}");
    // phi_x_omicron(2, SINGLETON_MIN, SINGLETON_LOGS);
    // // println!("{}..{mid}",SINGLETON_MIN+1);
    // phi_x_omicron(SINGLETON_MIN+1, mid, SINGLETON_LOGS);
    // // println!("{}..{max}", mid+1);
    // phi_x_omicron(mid+1, max, SINGLETON_LOGS);

fn phi_x_omicron(min: Int, max: Int, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        black_box(Batches::phi_equals_omicron(phi, 0).phi_x_omicron().phi_x_omicron());
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_2_n_phi_p_1_singleton() {
    let max = 100;
    if INCLUDE_PRE {
        phi_2_n_phi_p_1(2, SINGLETON_MIN, SINGLETON_LOGS);
    }
    if SPLIT {
        let mid: Int = (max+SINGLETON_MIN)/2;
        phi_2_n_phi_p_1(SINGLETON_MIN+1, mid, SINGLETON_LOGS);
        phi_2_n_phi_p_1(mid+1, max, SINGLETON_LOGS);
    } else {
        phi_2_n_phi_p_1(SINGLETON_MIN, max, SINGLETON_LOGS);
    }
}

fn phi_2_n_phi_p_1(min: Int, max: Int, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        black_box(Batches::phi_2_n_phi_p_1(phi, 0));
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn phi_2_singleton() {
    let max = 100;
    if INCLUDE_PRE {
        phi_2(2, SINGLETON_MIN, SINGLETON_LOGS);
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        phi_2(SINGLETON_MIN+1, mid, SINGLETON_LOGS);
        phi_2(mid+1, max, SINGLETON_LOGS);
    } else {
        phi_2(SINGLETON_MIN, max, SINGLETON_LOGS);
    }
}

fn phi_2(min: Int, max: Int, log: bool) -> Duration {
    let start = Instant::now();
    for phi in min..=max {
        black_box(Batches::phi_2(phi, 0));
    }
    let elapsed = start.elapsed();
    if log {
        log_total_and_mean(elapsed, (max - min + 1) as f64);
    }
    elapsed
}

#[test]
pub fn par_phi_2_comp_singleton() {
    let max = 100;
    if INCLUDE_PRE {
        par_phi_2_comp(2, SINGLETON_MIN, SINGLETON_LOGS);
        println!();
    }
    if SPLIT {
        let mid = (max+SINGLETON_MIN)/2;
        par_phi_2_comp(SINGLETON_MIN+1, mid, SINGLETON_LOGS);
        println!();
        par_phi_2_comp(mid+1, max, SINGLETON_LOGS);
    } else {
        par_phi_2_comp(SINGLETON_MIN, max, SINGLETON_LOGS);
    }
}

fn par_phi_2_comp(min: Int, max: Int, log: bool) -> (Duration, Duration) {
    let start = Instant::now();
    for phi in min..=max {
        black_box(Batches::sequential_phi_2_unchecked(phi, 0));
    }
    let seq_elapsed = start.elapsed();
    println!("seq");
    if log {
        log_total_and_mean(seq_elapsed, (max - min + 1) as f64);
    }
    
    let start = Instant::now();
    for phi in min..=max {
        black_box(Batches::parallel_phi_2_unchecked(phi, 0));
    }
    let par_elapsed = start.elapsed();
    println!("par");
    if log {
        log_total_and_mean(par_elapsed, (max - min + 1) as f64);
    }

    (seq_elapsed, par_elapsed)
}

#[test]
pub fn par_phi_2_alternate() {
    let min = 40;
    let max = 100;
    let iterations = 1;
    println!("\n");
    print!("par is faster: ");
    let mut a = 0.;
    for phi in min..max {
        if !phi.is_prime() {
            if phi == 100 {
                println!();
            }
            continue
        };
        let start = Instant::now();
        for offset in 0..iterations {
            black_box(Batches::sequential_phi_2_unchecked(phi, offset));
        }
        let seq_time = start.elapsed();
        let start = Instant::now();
        for offset in 0..iterations {
            black_box(Batches::parallel_phi_2_unchecked(phi, offset));
        }
        let par_time = start.elapsed();
        // if phi < 100 {
        //     if par_time < seq_time {
        //         print!("{phi}, ");
        //         stdout().flush().ok();
        //     }
        // } else {
        //     let par = par_time.as_secs_f64();
        //     let seq = seq_time.as_secs_f64();
        //     let p = (seq - par) / seq;
        //     a += p;
        //     println!("{phi}: (seq - par)/seq = {:.3}", p);
        //     println!("{a}");
        // }

        let par = par_time.as_secs_f64();
        let seq = seq_time.as_secs_f64();
        let p = (seq - par) / seq;
        a += p;
        println!("{phi}: (seq - par)/seq = {:.3}", p);
        println!("{a}");

        // if par_time < seq_time {
        //     print!("{phi}, ");
        //     stdout().flush().ok();
        // }
    }
    println!("\n");
}

#[test]
pub fn par_phi_2_n_phi_p_1_alternate() {
    let min = 40;
    let max = 100;
    let iterations = 1;
    println!("\n");
    println!("running");
    print!("par is faster: ");
    let mut a = 0.;
    for phi in min..max {
        if !(phi-1).is_prime() {
            // if phi == 100 {
            //     println!();
            // }
            continue
        };
        let start = Instant::now();
        for offset in 0..iterations {
            black_box(Batches::sequential_phi_2_n_phi_p_1_unchecked(phi, offset));
        }
        let seq_time = start.elapsed();
        let start = Instant::now();
        for offset in 0..iterations {
            black_box(Batches::parallel_phi_2_n_phi_p_1_unchecked(phi, offset));
        }
        let par_time = start.elapsed();
        // if phi < 100 {
        //     if par_time < seq_time {
        //         print!("{phi}, ");
        //         stdout().flush().ok();
        //     }
        // } else {
        //     let par = par_time.as_secs_f64();
        //     let seq = seq_time.as_secs_f64();
        //     let p = (seq - par) / seq;
        //     a += p;
        //     println!("{phi}: (seq - par)/seq = {:.3}", p);
        //     println!("{a}");
        // }

        let par = par_time.as_secs_f64();
        let seq = seq_time.as_secs_f64();
        let p = (seq - par) / seq;
        a += p;
        println!("{phi}|{}: (seq - par)/seq = {:.3}", phi - 1, p);
        println!("{a}");

        // if par_time < seq_time {
        //     print!("{phi}, ");
        //     stdout().flush().ok();
        // }
    }
    println!("\n");
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