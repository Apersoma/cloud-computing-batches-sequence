use crate::{binary_collections::BinaryResizableCollection, triples_array::TriplesArray};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestError;

 



///min and max will default to 2 and the max value that could work
pub fn test_omicron_quick(omicron: usize, min: Option<usize>, max: Option<usize>) -> Vec<(usize, Result<bool, TestError>)> {
    let min: usize = min.unwrap_or_default();
    let defaulted  = max.is_none();
    let max: usize = max.unwrap_or_else(||omicron.max_phi_weak()) + 1;
    let mut out = Vec::with_capacity(max - min + 1);
    for phi in min..=max{
        out.push((phi, omicron.test_quick(phi)));
    }
    if defaulted && min <= omicron {
        out.push((omicron, Ok(true)));
    }
    out
}

///min and max will default to 2 and the max value that could work
pub fn test_omicron_slow(omicron: usize, min: Option<usize>, max: Option<usize>, status_updates: bool) -> Vec<(usize, Result<bool, TestError>)> {
    let min = min.unwrap_or_default();
    let max: usize = max.unwrap_or_else(||omicron.max_phi_weak());
    
    let mut out = Vec::with_capacity(max - min + 1);
    for phi in min..=max{
        out.push((phi, omicron.test_slow(phi, status_updates)));
    }
    out
}

pub trait BatchNumber {
    fn test_quick(self, phi: Self) -> Result<bool,TestError>;
    fn max_phi_weak(self) -> Self;
    fn max_phi_strong(self) -> Self;
    fn test_slow(self, phi: Self, status_updates: bool) -> Result<bool, TestError>;
    fn inverse_phi_2_n_phi_p_1(self) -> Self; 
}

macro_rules! batch_number {
    ($t_up:ty) => {
        fn test_quick(self, phi: Self) -> Result<bool,TestError> {
            if self <= 1 {
                if phi == 0 {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }

            if phi <= 1 {return Ok(false)};
            if phi == 2 {return Ok(true)};
            if phi == self {return Ok(true)};
            if (self - 1) % (phi - 1) != 0 {return Ok(false)};
            if self == Self::MAX {panic!("not implemented")};
            if self < phi.saturating_mul(phi-1) {return Ok(false)};

            let m = phi*(phi-1);

            if self <= m {return Ok(false)};

            if (self % m)*((self-1) % m) % m > 0 {return Ok(false);}

            // let mut trivials: VecDeque<usize> = if phi == 3 {[3,13,15,19].into()} else {[phi].into()};


            let prime_phi = phi.is_prime();
            if !prime_phi && !(phi-1).is_prime() {
                return Err(TestError)
            }
            let mut trivials: Vec<Self> = if phi == 3 {[7,9,13,15,19,21].into()} else {[phi].into()};

            while !trivials.is_empty() {
                let cur = trivials.remove(0);
                if self == cur {
                    return Ok(true);
                }
                if (cur-1).is_prime() {
                    let new = cur.saturating_mul(cur-1).saturating_add(1);
                    if new <= self {
                        trivials.b_insert_keep(new);
                    }
                }
                if cur.is_prime() {
                    let new = cur.saturating_mul(cur);
                    if new <= self {
                        trivials.b_insert_keep(new);
                    }
                }
                // if prime_phi {
                    // let new = cur.saturating_mul(phi);
                    // if new <= omicron {
                    //     trivials.b_insert_keep(new);
                    // }
                // }
            }
            Err(TestError)
        }
        fn max_phi_strong(self) -> Self {
            let mut  max = self.max_phi_weak();
            while matches!(self.test_quick(max), Ok(false)) {
                max -= 1;
            }
            max
        }
        fn test_slow(self, phi: Self, status_updates: bool) -> Result<bool, TestError> {
            if phi == 3 {
                return Ok(TriplesArray::test(self as usize, status_updates))
            };
            self.test_quick(phi)
        }
        fn max_phi_weak(self) -> Self {
            if self == 2 {
                self
            } else {
                self.inverse_phi_2_n_phi_p_1()
            }
        }
        #[inline(always)]
        fn inverse_phi_2_n_phi_p_1(self) -> Self {
            (4*self as $t_up-3).isqrt().div_ceil(2) as Self
        }
    };
}

impl BatchNumber for usize {   
    #[cfg(target_pointer_width = "32")]
    batch_number!(u64);
    #[cfg(target_pointer_width = "64")]
    batch_number!(u128);
}

// impl BatchNumber for u64 {
//     batch_number!(u128);
// }

impl BatchNumber for u32 {
    batch_number!(u64);
}

// impl BatchNumber for u16 {
//     batch_number!(u32);
// }

// impl BatchNumber for u8 {
//     batch_number!(u16);
// }


pub fn format_test_results(results: Vec<(usize, Result<bool, TestError>)>, display_unknowns: bool) -> String {
    let mut out = String::with_capacity(results.len()/3 + 1);
    for result in results {
        match result {
            (num, Ok(true)) => out += &format!("{}, ", num),
            (_, Ok(false)) => (),
            (num, Err(TestError)) => if display_unknowns {out += &format!("?{}, ", num) },
        }
    }
    if out.ends_with(", ") {
        out.pop();
        out.pop();
    }
    out
}

pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char)
}

pub trait Composition {
    #[expect(clippy::wrong_self_convention)]
    fn is_prime(self) -> bool;
}

macro_rules! prime {
    ($t:ty) => {
        impl Composition for $t {
            fn is_prime(self) -> bool {
                if self <= 1 as $t {return false};
                if self & 1 as $t == 0 as $t {return self == 2 as $t};
                let mut f = 3 as $t;
                let sqrt = self.isqrt();
                while f <= sqrt {
                    unsafe {core::hint::assert_unchecked(f != 0)}
                    if self % f == 0 {
                        return false;
                    }
                    f = unsafe {f.unchecked_add(2 as $t)};
                } 
                true
            }
        }
    };
}

prime!(u8);
prime!(u16);
prime!(u32);
prime!(u64);
prime!(usize);
prime!(u128);
prime!(i8);
prime!(i16);
prime!(i32);
prime!(i64);
prime!(isize);
prime!(i128);

pub trait ExpectOr<A, E> {
    fn expect_or<'a, F: FnMut(E) -> &'a str>(self, func: F) -> A;
    fn expect_err_or<'a, F: FnMut(A) -> &'a str>(self, func: F) -> E;
}

impl<A, E> ExpectOr<A, E> for Result<A, E> {
    fn expect_or<'a, F: FnMut(E) -> &'a str>(self, mut func: F) -> A {
        match self {
            Ok(ans) => ans,
            Err(err) => panic!("{}", func(err))
        }
    }
    fn expect_err_or<'a, F: FnMut(A) -> &'a str>(self, mut func: F) -> E {
        match self {
            Ok(ans) => panic!("{}", func(ans)),
            Err(err) => err
        }
    }
}

pub trait UnwrapEither<T> {
    fn unwrap_either(self) -> T;
}

impl<T> UnwrapEither<T> for Result<T,T> {
    fn unwrap_either(self) -> T {
        match self {
            Ok(o) => o,
            Err(e) => e,
        }
    }
}