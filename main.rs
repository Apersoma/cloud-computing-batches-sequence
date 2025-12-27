#![feature(test)]
#![feature(stmt_expr_attributes)]
#![expect(clippy::manual_is_multiple_of)]
#![expect(unused_imports)]
// #[allow(internal_features)]
// #![feature(core_intrinsics)]


use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::mem;

pub mod batches_generator;
pub mod statics;
pub mod binary_collections;
pub mod triples_array;
pub mod batches;
mod tests;

use crate::statics::*;
use crate::binary_collections::*;
use crate::triples_array::*;
use crate::batches::*;

    // #[cfg(not(debug_assertions))]
    // tests::isqrt_or_f_x_f();

fn main() {
    println!("\nrunning\n");
    println!("\nA question mark preceding a number means it was unable to be determined if it was in the sequence or not\n");

    // let batches = Batches::phi_2_n_phi_p_1(2,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    // let batches = Batches::phi_2_n_phi_p_1(3,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    // let batches = Batches::phi_2_n_phi_p_1(4,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    // let batches = Batches::phi_2(2,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    // let batches = Batches::phi_2(3,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    // let batches = Batches::phi_2(5,0).unwrap();
    // println!("\n{}\n", batches.phi_x_omicron());
    for omicron in 2..50u32 {
        println!("{omicron} | {}", omicron.max_phi_weak());
    }
}

#[expect(non_snake_case, reason = "OEIS sequence")]
#[allow(dead_code)]
fn A386973_array() {
    let mut row = 2;
    while row < 15 {
        println!(
            "{: >2} : {}", 
            row, 
            format_test_results(
                test_omicron_slow(row, None, None, false),
                true
            )
        );
        row += 1;
    }
    while row < 30 {
        println!(
            "{: >2} : {}", 
            row, 
            format_test_results(
                test_omicron_quick(row, None, None),
                true
            )
        );
        row += 1;
    }
}