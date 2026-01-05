
extern crate test;
#[cfg(test)]
#[allow(unused_imports)]
use hashbrown::HashSet;
#[cfg(test)]
use rand::RngCore;
#[cfg(test)]
use rand::rngs::ThreadRng;
#[cfg(test)]
#[allow(unused_imports)]
use test::black_box;
#[cfg(test)]
use std::collections::BTreeSet;
#[cfg(test)]
#[allow(unused_imports)]
use std::env;
#[cfg(test)]
use crate::batches::Batches;
#[cfg(test)]
use crate::statics::*;
#[cfg(test)]
use crate::triples_array::TriplesArray;
#[cfg(test)]
#[allow(unused_imports)]
use crate::binary_collections::BinaryCollection;
//taken from A000040
#[cfg(test)]
const PRIMES: [u8; 54] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251];

#[test]
fn triples_array() {
    let correct = [3,7,9,13,15,19,21];
    for omicron in 3..=13 {
        let works = correct.contains(&omicron);
        assert_eq!(works, omicron.test_quick(3).unwrap());
        assert_eq!(works, omicron.test_slow(3, false).unwrap());
        let generation = TriplesArray::generate_stupid(omicron, true);
        if let Ok(generation) = generation {
            if works {
                Batches::from(generation).audit().unwrap();
            } else {
                assert_eq!(works, Batches::from(generation).audit().is_ok());
            }
        } else {
            assert_eq!(works, generation.is_ok());            
        }
    }
    for omicron in 14..25 {
        let works = correct.contains(&omicron);

        assert_eq!(works, TriplesArray::test(omicron, true));
        assert_eq!(works, omicron.test_quick(3).unwrap_or_default());
        assert_eq!(works, omicron.test_slow(3, false).unwrap());
        if works && omicron != 21 {
            let generation = TriplesArray::generate_stupid(omicron, true);
            Batches::from(generation.unwrap()).audit().unwrap();
        }
    }
}

#[test]
fn primes() {
    let mut primes = PRIMES.into_iter();
    let mut prime = primes.next();
    for x in 0..=u8::MAX {
        if prime.is_some_and(|p|p==x) {
            assert!(x.is_prime());
            prime = primes.next();
        } else {
            assert!(!x.is_prime());
        }
    }
    assert!(primes.len() == 0);
}

#[test]
fn test_phi_2_n_phi_p_1(){
    let mut primes = PRIMES.into_iter().map(|p|p as u32);
    let mut prime = Some(1);
    for phi in 2..=28 {
        print!("phi: {phi:?}");
        let omicron = phi*(phi-1)+1;
        let valid = omicron.test_quick(phi);
        if prime.is_some_and(|p|p==phi-1) {
            assert!(valid.is_ok(), "valid: {valid:?} | phi: {phi:?} | omicron: {omicron:?} | prime: {prime:?}");
            prime = primes.next();
        } else {
            assert!(valid.is_err(), "valid: {valid:?} | phi: {phi:?} | omicron: {omicron:?} | prime: {prime:?}");
        }

        print!("a");
        let batch0 = Batches::phi_2_n_phi_p_1(phi, 0);
        print!("b");
        let batch1 = Batches::phi_2_n_phi_p_1(phi, 1);
        print!("c");
        let batch2 = Batches::phi_2_n_phi_p_1(phi, 2);
        print!("d");        

        match valid {
            Ok(true) => {
                if phi < 24 {
                    print!("e");
                    batch0.unwrap().audit().unwrap();
                    print!("f");
                    batch1.unwrap().audit().unwrap();
                    print!("g");
                    batch2.unwrap().audit().unwrap();
                    print!("h");
                } else {
                    print!("e");
                    batch0.unwrap();
                    print!("f");
                    batch1.unwrap();
                    print!("g");
                    batch2.unwrap();
                    print!("h");
                }
            },
            Ok(false) => panic!(),
            Err(_) => {
                assert!(batch0.is_none());
                assert!(batch1.is_none());
                assert!(batch2.is_none());
            },
        }
        println!();
    }
}

#[test]
fn test_phi_2() {
    let mut primes = PRIMES.into_iter().map(|p|p as u32);
    let mut prime = primes.next();
    for phi in 2..=28 {
        print!("phi: {phi:?}");
        let omicron = phi*phi;
        let valid = omicron.test_quick(phi);
        if prime.is_some_and(|p|p==phi) {
            assert!(valid.is_ok(), "valid: {valid:?} | phi: {phi:?} | omicron: {omicron:?} | prime: {prime:?}");
            prime = primes.next();
        } else {
            assert!(valid.is_err(), "valid: {valid:?} | phi: {phi:?} | omicron: {omicron:?} | prime: {prime:?}");
        }

        print!("a");
        let batch0 = Batches::phi_2(phi, 0);
        print!("b");
        let batch1 = Batches::phi_2(phi, 1);
        print!("c");
        let batch2 = Batches::phi_2(phi, 2);
        print!("d");        

        match valid {
            Ok(true) => {
                if phi < 24 {
                    print!("e");
                    batch0.unwrap().audit().unwrap();
                    print!("f");
                    batch1.unwrap().audit().unwrap();
                    print!("g");
                    batch2.unwrap().audit().unwrap();
                    print!("h");
                } else {
                    print!("e");
                    batch0.unwrap();
                    print!("f");
                    batch1.unwrap();
                    print!("g");
                    batch2.unwrap();
                    print!("h");
                }
            },
            Ok(false) => panic!(),
            Err(_) => {
                assert!(batch0.is_none());
                assert!(batch1.is_none());
                assert!(batch2.is_none());
            },
        }
        println!();
    }
}

#[test]
fn test_phi_equal_omicron() {
    let mut rng = ThreadRng::default();
    rng.reseed().unwrap();
    for x in 2..=u8::MAX as u32 {
        assert_eq!(Batches::phi_equals_omicron(x, 0).to_string(), format!("{:?}", (0..x).collect::<BTreeSet<u32>>()));
        Batches::phi_equals_omicron(x, 0).audit().unwrap();
        Batches::phi_equals_omicron(x, 1).audit().unwrap();
        Batches::phi_equals_omicron(x, 2).audit().unwrap();
        Batches::phi_equals_omicron(x, 3).audit().unwrap();
        Batches::phi_equals_omicron(x, rng.next_u32() % u16::MAX as u32).audit().unwrap();
    }
}

/*
#[test]
fn test_phi_x_omicron() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
        println!("RUST_BACKTRACE")
    }
    let mut rng = ThreadRng::default();
    rng.reseed().unwrap();
    for phi in 2..4 {
        println!("phi: {phi}");
        #[expect(unused_labels)]
        'test_phi_equal_omicron: {
            let omicron = phi;
            println!("{:?}", test_quick(omicron*phi, phi));
            
            let batch0 = Batches::phi_equals_omicron(phi, 0);
            let batch0 = batch0.phi_x_omicron();
            let batch1 = Batches::phi_equals_omicron(phi, rng.next_u32() % u16::MAX as u3);
            let batch1 = batch1.phi_x_omicron();

            // println!("\n{batch0}\n");

            assert_eq!(batch0.phi, phi);
            assert_eq!(batch1.phi, phi);
            assert_eq!(batch0.omicron, omicron*phi);
            assert_eq!(batch1.omicron, omicron*phi);

            batch0.audit().unwrap();
            batch1.audit().unwrap();
        }
    }
    for phi in 2..8 {
        println!("phi: {phi}");
        'test_phi_2_n_phi_p_1: {
            let omicron = phi*(phi-1)+1;
            if !test_quick(omicron, phi).is_ok_and(|b|b) {
                break 'test_phi_2_n_phi_p_1
            }
            println!("{:?}", test_quick(omicron*phi, phi));

            
            let batch0 = Batches::phi_2_n_phi_p_1(phi, 0).unwrap();
            let batch0 = batch0.phi_x_omicron();
            // println!("\n{batch0}\n");
            let batch1 = Batches::phi_2_n_phi_p_1(phi, rng.next_u32() % u16::MAX as u3).unwrap();
            let batch1 = batch1.phi_x_omicron();

            assert_eq!(batch0.phi, phi);
            assert_eq!(batch1.phi, phi);
            assert_eq!(batch0.omicron, omicron*phi);
            assert_eq!(batch1.omicron, omicron*phi);

            batch0.audit().unwrap();
            batch1.audit().unwrap();
        }
    }
    for phi in 2..10 {
        println!("phi: {phi:?}");
        'test_phi_2: {
            let omicron = phi*phi;
            if !test_quick(omicron, phi).is_ok_and(|b|b) {
                break 'test_phi_2
            }
            println!("{:?}", test_quick(omicron*phi, phi));

            
            let batch0 = Batches::phi_2(phi, 0).unwrap();
            let batch0 = batch0.phi_x_omicron();
            // println!("\n{batch0}\n");
            let batch1 = Batches::phi_2(phi, rng.next_u32() % u16::MAX as u3).unwrap();
            let batch1 = batch1.phi_x_omicron();

            assert_eq!(batch0.phi, phi);
            assert_eq!(batch1.phi, phi);
            assert_eq!(batch0.omicron, omicron*phi);
            assert_eq!(batch1.omicron, omicron*phi);

            batch0.audit().unwrap();
            batch1.audit().unwrap();
        }
        
    }
}
*///*/
//*


#[cfg(not(debug_assertions))]
// #[test]
#[allow(unused)]
pub fn isqrt_or_f_x_f() {
    sleep(Duration::from_millis(500));
    let f_x_f_start = Instant::now();
    for _ in 0..500 {
        for x in 1..=u16::MAX as usize {
            let mut f = 3;
            while f*f < x {
                f += 2;
                black_box(f);
            }
        }
    }
    let f_x_f_time = f_x_f_start.elapsed();

    // sleep(Duration::from_secs(1));

    let isqrt_start = Instant::now();
    for _ in 0..500 {
        for x in 1..=u16::MAX as usize {
            let mut f = 3;
            let sqrt = x.isqrt();
            while f < sqrt {
                f += 2;
                black_box(f);
            }
            black_box(sqrt);
        }
    }
    let isqrt_time = isqrt_start.elapsed();

    println!("f_x_f duration: {}", f_x_f_time.as_secs());
    println!("f_x_f duration: {}", f_x_f_time.as_millis());
    println!("f_x_f duration: {}", f_x_f_time.as_nanos());
    println!("isqrt duration: {}", isqrt_time.as_secs());
    println!("isqrt duration: {}", isqrt_time.as_millis());
    println!("isqrt duration: {}", isqrt_time.as_nanos());
}