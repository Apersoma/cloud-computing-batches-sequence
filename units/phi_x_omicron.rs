#![cfg(test)]

use std::time::Instant;
use std::collections::BTreeSet;

use itertools::Itertools;

use crate::batches::*;
use crate::statics::*;
use super::benches::*;
use crate::triples_array::TriplesArray;
use crate::units::stepped_iter;
use crate::Int;

#[test]
fn phi_x_omicron_general() {
    println!("\n");
    // unsafe {
    //     env::set_var("RUST_BACKTRACE", "1");
    //     println!("RUST_BACKTRACE")
    // }
    println!();
    let mut valid: BTreeSet<[Int; 2]> = BTreeSet::new();
    let mut invalid: BTreeSet<[Int; 2]> = BTreeSet::new();

    println!("triples_array");
    let start = Instant::now();
    for mut omicron in [13, 15, 19usize] {
        // println!("omicron: {omicron}");
        let phi = 3usize;        
        match omicron.test_quick(phi) {
            Ok(true) => (),
            Ok(false) => {
                panic!();
            }
            Err(_) => panic!()
        }
        
        let mut batches: Batches = TriplesArray::generate_stupid(omicron, false).unwrap().into();
        batches = batches.phi_x_omicron();

        match (batches.audit(), batches.omicron.test_quick(3)) {
            (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([3, omicron as Int])),
            _ => panic!(),
        }

        for _ in 0..3 {
            // println!("|");

            omicron *= phi;
            // if let Ok(ans) = (omicron*phi).test_quick(phi) {
            //     println!("{ans}");
            // }

            batches = batches.phi_x_omicron();

            match (batches.audit(), batches.omicron.test_quick(phi as Int)) {
                (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([phi as Int, omicron as Int]), "{phi}, {omicron}"),
                (Err(_), Err(_)) => {
                    // println!("{err}");
                    assert!(invalid.insert([phi as Int, omicron as Int]));
                    break;
                },
                (Err(err), Ok(false)) => panic!("{err}"),
                // (Err(_), Ok(false)) => assert!(invalid_expected.insert([phi, omicron])),
                (Err(err), Ok(true)) => panic!("{err}"),
                (Ok(_), Ok(false)) => panic!(),
            }
        }
    }
    print_elapsed(start);

    println!("\n");

    println!("phi == omicron");
    let start = Instant::now();
    for (iterations, phi) in stepped_iter(&[2,3,15,40]) {
        let mut omicron = phi;
        match omicron.test_quick(phi) {
            Ok(true) => (),
            Ok(false) => {panic!();}
            Err(_) => panic!(),
        }
        
        let mut batches = Batches::phi_equals_omicron(phi, 0);
        batches = batches.phi_x_omicron();

        match (batches.audit(), batches.omicron.test_quick(phi)) {
            (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([phi, omicron])),
            (Err(_), Err(_)) => {
                assert!(invalid.insert([phi, omicron]));
                continue;
            },
            (Err(err), Ok(false)) => panic!("{err}"),
            (Err(err), Ok(true)) => {
                println!("{batches:?}");
                panic!("{err}")
            },
            (Ok(_), Ok(false)) => panic!(),
        }
        
        for _ in 0..iterations+3*(phi==2) as usize {
            omicron *= phi;
            batches = batches.phi_x_omicron();

            match (batches.audit(), batches.omicron.test_quick(phi)) {
                (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([phi, omicron]), "{phi}, {omicron}, {iterations}"),
                (Err(_), Err(_)) => {
                    // println!("{err}");
                    assert!(invalid.insert([phi, omicron]));
                    break;
                },
                (Err(err), Ok(false)) => panic!("{err}"),
                // (Err(_), Ok(false)) => assert!(invalid_expected.insert([phi, omicron])),
                (Err(err), Ok(true)) => panic!("{err}"),
                (Ok(_), Ok(false)) => panic!(),
            }
        }
    }
    print_elapsed(start);

    println!("\n");

    println!("phi*phi - phi + 1 = omicron");
    let start = Instant::now();
    for (iterations, phi) in stepped_iter(&[2,3,16,17,18]) {
        let iterations  = iterations + 2;
        let mut omicron = phi*(phi-1)+1;
        // println!("phi: {phi} | iterations: {iterations}");
        match omicron.test_quick(phi) {
            Ok(true) => (),
            Ok(false) => panic!(),
            Err(_) => continue,
        }
        
        let mut batches = Batches::phi_2_n_phi_p_1(phi, 0).unwrap();
        
        batches = batches.phi_x_omicron();
        match (batches.audit(), batches.omicron.test_quick(phi)) {
            (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([phi, omicron]), "{phi}, {omicron}, {iterations}"),
            (Err(ValidationError{severe: false, ..}), Err(_)) => {
                assert!(invalid.insert([phi, omicron]));
                continue;
            },
            (Err(ValidationError{severe: true, err}), Err(_)) => {
                println!("{batches:?}");
                panic!("{}", ValidationError{severe: true, err})
            },
            (Err(err), Ok(false)) => panic!("{err}"),
            (Err(err), Ok(true)) => panic!("{err}"),
            (Ok(_), Ok(false)) => panic!(),
        }
        
        for _ in 0..iterations {

            omicron *= phi;

            batches = batches.phi_x_omicron();

            match (batches.audit(), batches.omicron.test_quick(phi)) {
                (Ok(_), Ok(true)) | (Ok(_), Err(_)) => assert!(valid.insert([phi, omicron]), "{phi}, {omicron}, {iterations}"),
                (Err(_), Err(_)) => {
                    assert!(invalid.insert([phi, omicron]));
                    break;
                },
                (Err(err), Ok(false)) => panic!("{err}"),
                (Err(err), Ok(true)) => panic!("{err}"),
                (Ok(_), Ok(false)) => panic!(),
            }
        }
    }
    print_elapsed(start);
    println!("\n");

    println!(
        "valid: {}", 
        format!("{valid:?}")
            .replace("[", "\\left(")
            .replace("]", "\\right)")
            .replacen("{", "\\left[", 1)
            .replacen("}", "\\right]", 1)
    );
    println!();
    println!(
        "invalid: {}", 
        format!("{invalid:?}")
            .replace("[", "\\left(")
            .replace("]", "\\right)")
            .replacen("{", "\\left[", 1)
            .replacen("}", "\\right]", 1)
    );
    println!();

    for [phi, omicron] in invalid.iter() {
        assert!(!invalid.contains(&[*phi, omicron * phi]), "{:?}", [phi, omicron]);
        assert!(!valid.contains(&[*phi, omicron * phi]), "{:?}", [phi, omicron]);
    }
}
//*/

/*
#[test]
fn phi_x_omicron_single() {
    println!("\n");
    // unsafe {
    //     env::set_var("RUST_BACKTRACE", "1");
    //     println!("RUST_BACKTRACE")
    // }
    println!();
    let mut valid: BTreeSet<[Int; 2]> = BTreeSet::new();
    let mut invalid: BTreeSet<[Int; 2]> = BTreeSet::new();
    // let mut valid: BTreeSet<[Int; 2]> = BTreeSet::from([[2, 3], [2, 4], [3, 7], [3, 9], [4, 13], [5, 25], [6, 31], [7, 49], [11, 121], [13, 169], [17, 289], [18, 307], [19, 361], [23, 529]]);
    // let mut invalid: BTreeSet<[Int; 2]> = BTreeSet::from([[8, 57], [12, 133], [14, 183], [20, 381], [24, 553]]);
    let init = valid.is_empty();

    fn checkups(valids: bool, init: bool, valid: &BTreeSet<[Int; 2]>, invalid: &BTreeSet<[Int; 2]>) {
        if !init {
            if valids {
                println!("valid: {valid:?}");
                println!("valid: phi: {:?}", valid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
                println!("valid: omicron: {:?}", valid.iter().map(|p|p[1]).collect::<Vec<Int>>());
            } else {
                println!("invalid: {invalid:?}");
                println!("invalid: phi: {:?}", invalid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
                println!("invalid: omicron: {:?}", invalid.iter().map(|p|p[1]).collect::<Vec<Int>>());
            }
        }
    }
    
    println!("loop");
    let start = Instant::now();

    #[allow(clippy::reversed_empty_ranges)]
    for phi in 2..20 {
        println!("phi: {phi}");
        if phi <= 3 {
            valid.insert([phi, phi*phi - phi + 1]);
            valid.insert([phi, phi*phi]);
            continue;
        }

        let omicron;
        let Some(mut batches) = ( 
            if phi & 1 == 1 {
                omicron = phi*phi;
                Batches::phi_2(phi, 0)
            } else {
                omicron = phi*phi - phi + 1;
                Batches::phi_2_n_phi_p_1(phi, 0)
            }
        ) else {continue};
        println!(".");
        // batches = batches.phi_x_omicron();
        // match (batches.audit(), batches.omicron.test_quick(phi)) {
        match (batches.phi_x_omicron_test(), (phi * batches.omicron).test_quick(phi)) {
            (Ok(_), Ok(true)) | (Ok(_), Err(_)) => {
                assert!(valid.insert([phi, omicron]), "{phi}; {omicron}");
                checkups(true, init, &valid, &invalid);
            },
            (Err(ValidationError{severe: false, ..}), Err(_)) => {
                assert!(invalid.insert([phi, omicron]));
                checkups(false, init, &valid, &invalid);
            },
            (Err(err), Ok(false)) =>
                panic!("{err}"),
            (Err(err), Ok(true)) => {
                println!("{batches:?}");
                panic!("{err}")
            },
            (Err(ValidationError{severe: true, err}), Err(_)) => {
                println!("{batches:?}");
                panic!("{}", ValidationError{severe: true, err})
            },
            (Ok(_), Ok(false)) => 
                panic!(),
        }
        
    }
    print_elapsed(start);
    println!("\n");
    
    // println!(
    //     "valid: {}", 
    //     format!("{valid:?}")
    //         .replace("[", "\\left(")
    //         .replace("]", "\\right)")
    //         .replacen("{", "\\left[", 1)
    //         .replacen("}", "\\right]", 1)
    // );
    // println!();
    // println!(
    //     "invalid: {}", 
    //     format!("{invalid:?}")
    //         .replace("[", "\\left(")
    //         .replace("]", "\\right)")
    //         .replacen("{", "\\left[", 1)
    //         .replacen("}", "\\right]", 1)
    // );
    // println!();

    println!("valid: {:?}", valid);
    println!("invalid: {:?}", invalid);

    println!("valid phi: {:?}", valid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
    println!("invalid phi: {:?}", invalid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());

    println!("valid omicron: {:?}", valid.iter().map(|p|p[1]).collect::<Vec<Int>>());
    println!("invalid omicron: {:?}", invalid.iter().map(|p|p[1]).collect::<Vec<Int>>());

    println!();

    println!("valid phi: \n 4,5,6,7 -seq:8 seq:11 -seq:12 seq:13 -seq:14 seq:17 seq:18 seq:19 -seq:20 seq:23 -seq:24 \n");
    println!("invalid phi: \n 12, 14 -seq:4 -seq:5 -seq:6 -seq:7 seq:8 -seq:11 -seq:13 -seq:17 -seq:18 -seq:19 seq:20 -seq:23 seq:24 \n");

}
*/
//*/
//no collect 28.00, 27.79, 27.15
//BinaryHeap: 27.55, 29.08, 30.38
#[test]
fn phi_x_omicron_single() {
    println!("\n");
    // unsafe {
    //     env::set_var("RUST_BACKTRACE", "1");
    //     println!("RUST_BACKTRACE")
    // }
    println!();
    let mut valid: BTreeSet<[Int; 2]> = BTreeSet::new();
    let mut invalid: BTreeSet<[Int; 2]> = BTreeSet::new();
    // let mut valid: BTreeSet<[Int; 2]> = BTreeSet::from([[2, 3], [2, 4], [3, 7], [3, 9], [4, 13], [5, 25], [6, 31], [7, 49], [11, 121], [13, 169], [17, 289], [18, 307], [19, 361], [23, 529]]);
    // let mut invalid: BTreeSet<[Int; 2]> = BTreeSet::from([[8, 57], [12, 133], [14, 183], [20, 381], [24, 553]]);
    let init = valid.is_empty();

    fn checkups(valids: bool, init: bool, valid: &BTreeSet<[Int; 2]>, invalid: &BTreeSet<[Int; 2]>) {
        if !init {
            if valids {
                println!("valid: {valid:?}");
                println!("valid: phi: {:?}", valid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
                println!("valid: omicron: {:?}", valid.iter().map(|p|p[1]).collect::<Vec<Int>>());
            } else {
                println!("invalid: {invalid:?}");
                println!("invalid: phi: {:?}", invalid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
                println!("invalid: omicron: {:?}", invalid.iter().map(|p|p[1]).collect::<Vec<Int>>());
            }
        }
    }
    
    println!("loop");
    let start = Instant::now();

    #[allow(clippy::reversed_empty_ranges)]
    for phi in 2..18 {
        println!("phi: {phi}");
        if phi <= 3 {
            valid.insert([phi, phi*phi - phi + 1]);
            valid.insert([phi, phi*phi]);
            continue;
        }

        let omicron = 
            if phi & 1 == 1 {
                if !phi.is_prime() {continue};
                phi*phi
            } else {
                if !(phi-1).is_prime() {continue};
                phi*phi - phi + 1
            };
        // let Some(batches) = ( 
        //     if phi & 1 == 1 {
        //         omicron = phi*phi;
        //         Batches::phi_2(phi, 0)
        //     } else {
        //         omicron = phi*phi - phi + 1;
        //         Batches::phi_2_n_phi_p_1(phi, 0)
        //     }
        // ) else {continue};
        println!(".");
        // batches = batches.phi_x_omicron();
        // match (batches.audit(), batches.omicron.test_quick(phi)) {
        match (phi_x_omicron_test(phi, omicron), (phi * omicron).test_quick(phi)) {
            (true, Ok(true)) | (true, Err(_)) => {
                assert!(valid.insert([phi, omicron]), "{phi}; {omicron}");
                checkups(true, init, &valid, &invalid);
            },
            (false, Err(_)) => {
                assert!(invalid.insert([phi, omicron]));
                checkups(false, init, &valid, &invalid);
            },
            (false, Ok(false)) =>
                panic!(),
            (false, Ok(true)) => {
                panic!()
            },
            (true, Ok(false)) => 
                panic!(),
        }
        
    }
    print_elapsed(start);
    println!("\n");
    
    // println!(
    //     "valid: {}", 
    //     format!("{valid:?}")
    //         .replace("[", "\\left(")
    //         .replace("]", "\\right)")
    //         .replacen("{", "\\left[", 1)
    //         .replacen("}", "\\right]", 1)
    // );
    // println!();
    // println!(
    //     "invalid: {}", 
    //     format!("{invalid:?}")
    //         .replace("[", "\\left(")
    //         .replace("]", "\\right)")
    //         .replacen("{", "\\left[", 1)
    //         .replacen("}", "\\right]", 1)
    // );
    // println!();

    println!("valid: {:?}", valid);
    println!("invalid: {:?}", invalid);

    println!("valid phi: {:?}", valid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());
    println!("invalid phi: {:?}", invalid.iter().map(|p|p[0]).dedup().collect::<Vec<Int>>());

    println!("valid omicron: {:?}", valid.iter().map(|p|p[1]).collect::<Vec<Int>>());
    println!("invalid omicron: {:?}", invalid.iter().map(|p|p[1]).collect::<Vec<Int>>());

    println!();

    println!("valid phi: \n 4,5,6,7 -seq:8 seq:11 -seq:12 seq:13 -seq:14 seq:17 seq:18 seq:19 -seq:20 seq:23 -seq:24 \n");
    println!("invalid phi: \n 12, 14 -seq:4 -seq:5 -seq:6 -seq:7 seq:8 -seq:11 -seq:13 -seq:17 -seq:18 -seq:19 seq:20 -seq:23 seq:24 \n");

}

fn phi_x_omicron_test(phi: Int, base_omicron: Int) -> bool {
    let pair_count = phi as usize * (phi as usize - 1) / 2 * base_omicron as usize * base_omicron as usize;
    let mut pairs: hashbrown::HashSet<[Int; 2], rustc_hash::FxBuildHasher> = hashset(pair_count);
    
    for i in 0..base_omicron {
        for ii in 0..base_omicron {
            let set = (1..phi)
                .map(|iii| base_omicron*iii + ((i*iii+ii) % base_omicron))
                .collect::<Vec<Int>>();
            for iii in 0..set.len() {
                let x = set[iii];
                #[expect(clippy::needless_range_loop)]
                for iv in 0..iii {
                    if !pairs.insert([x, set[iv]]) {
                        return false;
                    }
                }
                if !pairs.insert([i, x]) {
                    panic!();
                    #[expect(unreachable_code)]
                    return false;
                }
            }
            
            // let mut iter = (1..phi);
            // while let Some(x) = iter.next() {
            //     for y in iter.clone() {
            //         if !pairs.insert([x, y]) {
            //             return false;
            //         }
            //     }
            //     if !pairs.insert([i, x]) {
            //         return false;
            //     }
            // }
        }
    }

    debug_assert_eq!(pair_count, pairs.len());
    true
}