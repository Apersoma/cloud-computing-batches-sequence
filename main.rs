#![feature(test)]
#![feature(stmt_expr_attributes)]
#![expect(clippy::manual_is_multiple_of)]
// #![expect(unused_imports)]
#![feature(iter_map_windows)]
#![feature(step_trait)]
// #[allow(internal_features)]
// #![feature(core_intrinsics)]

#[allow(unused_imports)]
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::collections::VecDeque;
#[allow(unused_imports)]
use std::mem;

pub mod batches_generator;
pub mod statics;
pub mod binary_collections;
pub mod triples_array;
pub mod batches;
pub mod signed_unsigned;
pub mod grown_shrunk;
mod tests;
mod units;



use crate::statics::*;
#[allow(unused_imports)]
use crate::binary_collections::*;
#[allow(unused_imports)]
use crate::triples_array::*;
#[allow(unused_imports)]
use crate::batches::*;

    // #[cfg(not(debug_assertions))]
    // tests::isqrt_or_f_x_f();
    
pub type Int = u32;

fn main() {
    let phi= 11;
    let offset = 0;
    // let omicron = phi*phi;

    // let mut sets = hashset(omicron as usize + phi as usize);

    let phi_n1 = phi-1;

    let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;
    
    // let mut iter0 = (0..phi+1)
    // #[cfg(target_pointer_width = "64")]
    let mut iter0 = (1..=phi)
        .map(|i|{
            let mut mini_set = BTreeSet::new();
            insert_unique_btree!(mini_set, offset);
            for ii in 1..phi {
                insert_unique_btree!(mini_set, indices_to_base_value(i,ii));
            }
            mini_set
        });
    let mut iter1 = (1..phi)
        .flat_map(|i|(1..=phi)
        // .flat_map(|i|(1..=phi)
            .map(move |ii| {
                let mut mini_set = BTreeSet::new();
                insert_unique_btree!(mini_set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(mini_set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi)+1,iii));
                }
                mini_set
            })
        );
    let mut iter2 = (1..phi)
        .map(|i|{
            let mut mini_set = BTreeSet::new();
            for ii in 1..=phi {
                insert_unique_btree!(mini_set, indices_to_base_value(ii, i));
            }
            mini_set
        });
    
    let _: Vec<&mut (dyn Iterator<Item = _>+Send)> = vec![
        &mut iter0,
        &mut iter1,
        &mut iter2,
    ];
    // let result: Vec<_> = iters.into_par_iter().flatten_iter().collect();
    //for each would then be called 
}
    // let iters: VecDeque<&mut dyn ExactSizeIterator<Item = _>> = iters.into_iter().collect::<VecDeque<_>>();
    // let iters: Vec<&mut dyn ExactSizeIterator<Item = _>> = vec![
    //     &mut iter0,
    //     // &mut iter1,
    //     &mut iter2,
    // ];
    //vec![.....].into_par_iter().flatten_iter().collect()}

/*
sudo code for the multithreading
let set = HashSet::new();

let thread_0 = Thread::from(||
    HashSet::from_iter(foo_0())
);
thread_0.start();
let thread_1 = Thread::from(||
    HashSet::from_iter(foo_1())
);
thread_1.start();
let thread_2 = Thread::from(||
    HashSet::from_iter(foo_1())
);
thread_2.start();

let mut thread_0_running = true;
let mut thread_1_running = true;
let mut thread_2_running = true;

while thread_0_running || 
    thread_1_running ||
    thread_2_running 
{
    if thread_0_running && thread_0.finished() 
    {
        for val in thread_0.result() {
            set.insert(val);
        }
    }
    if thread_1_running && thread_1.finished() 
    {
        for val in thread_1.result() {
            set.insert(val);
        }
    }
    if thread_2_running && thread_2.finished() 
    {
        for val in thread_2.result() {
            set.insert(val);
        }
    }
}
*/

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