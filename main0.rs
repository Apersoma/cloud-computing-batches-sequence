//#![allow(unused)]
#![allow(unused_imports)]
use std::mem;
mod utils {
    use std::cmp::Ordering;
    use std::ops::Index;
    use std::ops::IndexMut;
    pub trait BinaryCollection<Element> {
        fn b_insert_before(&mut self, e: Element);

        fn b_insert_after(&mut self, e: Element);

        fn b_insert_replace(&mut self, e: Element) -> Option<Element>;

        fn b_insert_keep(&mut self, e: Element) -> Option<&Element>;

        /// Returns the index of the greatest element less than or equal to e
        fn b_search_less(&self, e: &Element) -> usize;

        /// Returns the index of the least element greater than or equal to e
        fn b_search_greater(&self, e: &Element) -> usize;

        /// Returns the index of e, None if it is not in there
        fn b_search(&self, e: &Element) -> Option<usize>;
    }

    impl<E> BinaryCollection<E> for Vec<E> where E: Ord+Sized {
        fn b_insert_before(&mut self, e: E) {
            self.insert(self.b_search_greater(&e), e);
        }
    
        fn b_insert_after(&mut self, e: E) {
            self.insert(self.b_search_greater(&e).min(self.len()-1)+1, e);
        }
    
        fn b_insert_replace(&mut self, mut e: E) -> Option<E> {
            let i = self.b_search_greater(&e);
            if i==self.len() {
                self.push(e);
                return None;
            }
            
            let x = &mut self[i];
            if *x == e  {
                std::mem::swap(x, &mut e);
                return Some(e);
            }

            self.insert(i, e);
            return None
        }
    
        fn b_insert_keep(&mut self, e: E) -> Option<&E> {
            let i = self.b_search_greater(&e);
            // println!("i:{}",i);
            if i==self.len() {
                self.push(e);
                return None;
            }
            
            if self[i] == e {
                return Some(&self[i]);
            }

            self.insert(i, e);
            return None
        }
        
        fn b_search_less(&self, e: &E) -> usize {
            let mut low = 0;
            let mut high = self.len()-1;
        
            while low <= high {
                let mid = low + (high-low)/2;
                match self[mid].cmp(e) {
                    Ordering::Equal => return mid,
                    Ordering::Less => low = mid+1,
                    Ordering::Greater => high = mid-1,
                }
            }
            return high
        }
        
        fn b_search_greater(&self, e: &E) -> usize {
            if self.len() == 0 {return 0};
            let mut low = 0;
            let mut high = self.len()-1;
        
            while low <= high {
                let mid = low + (high-low)/2;
                match self[mid].cmp(e) {
                    Ordering::Equal => return mid,
                    Ordering::Less => low = mid+1,
                    Ordering::Greater => {if mid == 0 {return 0} else {high = mid-1}},
                }
            }
            return low
        }
        
        fn b_search(&self, e: &E) -> Option<usize> {
            let mut low = 0;
            let mut high = self.len()-1;
        
            while low <= high {
                let mid = low + (high-low)/2;
                match self[mid].cmp(e) {
                    Ordering::Equal => return Some(mid),
                    Ordering::Less => low = mid+1,
                    Ordering::Greater => high = mid-1,
                }
            }
            return None
        }
    }
}
use utils::*;
mod math {

    //use std::mem;
    use consts::E;
    use std::collections::VecDeque;
    use std::f64::consts;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    use std::{fmt, fs::OpenOptions};

    use crate::utils::{self, BinaryCollection};

    // #[derive(Debug, Clone, PartialEq)]
//     struct PreviousValsRecursive {
//         vals: Vec<f64>,
//         ans: Option<f64>,
//         min: f64,
//         max: f64,
//         prev: Option<f64>,
//         prev_prev: Option<f64>,
//     }

//     impl std::fmt::Display for PreviousValsRecursive {
//         fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//             let vals = &self.vals;
//             let mut k = "values: [".to_owned();
//             if vals.len() == 0 {
//                 k = k.to_owned() + "]";
//             } else {
//                 for i in 0..(vals.len() - 1) {
//                     k = k.to_owned() + &vals[i].to_string() + ",";
//                 }
//                 k = k.to_owned() + &vals[vals.len() - 1].to_string() + "] ";
//             }

//             match self.ans {
//                 None => k = k.to_owned() + "ans: None",
//                 Some(x) => k = k.to_owned() + "ans: " + &x.to_string(),
//             }

//             match self.prev {
//                 None => k = k.to_owned() + " prev: None",
//                 Some(x) => k = k.to_owned() + " prev: " + &x.to_string(),
//             }

//             k = k.to_owned()
//                 + " interval: ["
//                 + &self.min.to_string()
//                 + ", "
//                 + &self.max.to_string()
//                 + "]";
//             write!(f, "{}", k)
//         }
//     }

//     impl PreviousValsRecursive {
//         fn push(&mut self, value: f64) {
//             if self.ans.is_some() {
//                 return;
//             }
//             let prev_exists = self.prev.is_some();
//             let prev_prev_exists = self.prev_prev.is_some();
//             let oob = value < self.min || value > self.max;
//             // let min = &self.min;
//             // let max = &self.max;

//             //let set_prev = |value, prev_vals: &mut PreviousValsRecursive| {prev_vals.prev_prev = prev_vals.prev; prev_vals.prev = value;};

//             if self.vals.len() == 0 && !prev_exists {
//                 if oob {
//                     self.prev_prev = self.prev;
//                     self.prev = Some(value);
//                 } else {
//                     self.vals.push(value);
//                     self.prev_prev = self.prev;
//                     self.prev = None;
//                 }
//                 return;
//             }

//             let prev_value = self.prev_value();
//             let vals = &mut self.vals;

//             if vals.len() == 1 {
//                 if oob {
//                     if let Some(prev_prev_value) = self.prev_prev {
//                         if prev_prev_value == value {
//                             self.ans = Some(value);
//                             return;
//                         }

//                         if (value < prev_value) ^ (prev_value < prev_prev_value)
//                             && (value < self.min) == (prev_value < self.min)
//                             && (prev_value < self.min) == (prev_prev_value < self.min)
//                         {
//                             self.ans = Some(value);
//                             return;
//                         }
//                     }
//                     self.prev_prev = self.prev;
//                     self.prev = Some(value);
//                 } else {
//                     vals.push(value);
//                     self.prev_prev = self.prev;
//                     self.prev = None;
//                 }
//                 return;
//             }

//             if prev_value == value {
//                 self.ans = Some(value);
//                 return;
//             }

//             if prev_exists {
//                 if oob {
//                     if prev_prev_exists || vals.len() > 0 {
//                         let prev_prev_value = self.prev_prev_value();

//                         if prev_prev_value == value {
//                             self.ans = Some(value);
//                             return;
//                         }

//                         if (value < prev_value) ^ (prev_value < prev_prev_value)
//                             && (value < self.min) == (prev_value < self.min)
//                             && (prev_value < self.min) == (prev_prev_value < self.min)
//                         {
//                             self.ans = Some(if value < self.min {
//                                 (self.min + value) / 2f64
//                             } else {
//                                 (self.max + value) / 2f64
//                             });
//                             return;
//                         }
//                     }
//                     self.prev_prev = self.prev;
//                     self.prev = Some(value);
//                     return;
//                 }

//                 if vals.len() > 0 {
//                     for i in vals.len() - 1..0 {
//                         if value == vals[i] {
//                             self.ans = Some(value);
//                             return;
//                         }
//                     }
//                 }
//                 self.prev_prev = self.prev;
//                 self.prev = None;
//                 vals.push(value);
//                 return;
//             }

//             // if let Some(prev_prev_value) = self.prev_prev {
//             //     if prev_prev_value == value {
//             //         self.ans = Some(value);
//             //         return;
//             //     }
//             // }

//             if prev_value < value {
//                 self.min = prev_value;
//                 if vals.len() >= 2 {
//                     //println!("lbl 1");
//                     let mut i = vals.len() - 2;
//                     loop {
//                         if vals[i] < self.min {
//                             vals.swap_remove(i);
//                         }
//                         if i == 0 {
//                             break;
//                         }
//                         i -= 1;
//                     }
//                 }
//                 if value > self.max {
//                     self.prev_prev = self.prev;
//                     self.prev = Some(value);
//                     return;
//                 } else {
//                     vals.push(value);
//                 }
//             } else if prev_value > value {
//                 self.max = prev_value;
//                 if vals.len() >= 2 {
//                     let mut i = vals.len() - 2;
//                     loop {
//                         if vals[i] > self.max {
//                             vals.remove(i);
//                         }
//                         if i <= 0 {
//                             break;
//                         }
//                         i -= 1;
//                     }
//                 }
//                 if value < self.min {
//                     self.prev = Some(value);
//                     return;
//                 } else {
//                     vals.push(value);
//                 }
//             } else {
//                 self.ans = Some(value);
//                 return;
//             }

//             if vals.len() >= 3 {
//                 let mut i = vals.len() - 3;
//                 loop {
//                     if value == vals[i] {
//                         self.ans = Some(value);
//                         return;
//                     }
//                     if i == 0 {
//                         break;
//                     } else {
//                         i -= 1;
//                     }
//                 }
//             }
//         }

//         fn new_in_range(min: f64, max: f64) -> PreviousValsRecursive {
//             return PreviousValsRecursive {
//                 vals: Vec::with_capacity(3usize),
//                 ans: None,
//                 min: min,
//                 max: max,
//                 prev: None,
//                 prev_prev: None,
//             };
//         }

//         fn new() -> PreviousValsRecursive {
//             return Self::new_in_range(f64::NEG_INFINITY, f64::INFINITY);
//         }

//         fn new_in_range_with_1st(min: f64, max: f64, x: f64) -> PreviousValsRecursive {
//             let mut k = Self::new_in_range(min, max);
//             if x < min || x > max {
//                 k.prev = Some(x);
//             } else {
//                 k.push(x);
//             }
//             return k;
//         }

//         fn new_with_1st(x: f64) -> PreviousValsRecursive {
//             return Self::new_in_range_with_1st(f64::NEG_INFINITY, f64::INFINITY, x);
//         }

//         fn prev_value(&self) -> f64 {
//             return match self.prev {
//                 Some(prev) => prev,
//                 None => self.vals[self.vals.len() - 1],
//             };
//         }

//         fn prev_prev_value(&self) -> f64 {
//             if let Some(value) = self.prev_prev {
//                 return value;
//             } else if self.prev.is_some() {
//                 return self.vals[self.vals.len() - 1];
//             } else {
//                 return self.vals[self.vals.len() - 2];
//             }
//         }

//         fn prev_option(&self) -> Option<&f64> {
//             if self.prev.is_some() {
//                 return self.prev.as_ref();
//             } else {
//                 return self.vals.get(self.vals.len() - 1);
//             }
//         }

//         fn prev_prev_option(&self) -> Option<&f64> {
//             if self.prev_prev.is_some() {
//                 return self.prev_prev.as_ref();
//             } else if self.prev.is_some() {
//                 return self.vals.get(self.vals.len() - 1);
//             } else {
//                 return self.vals.get(self.vals.len() - 2);
//             }
//         }
//     }

//     impl FloatTraits<i64> for f64 {
//         const MANTISSA_BIAS: i64 = 1023;

//         fn mantissa_biased(self) -> i64 {
//             return (self.to_bits() >> 52 & 0b11111111111) as i64;
//         }

//         fn mantissa(self) -> i64 {
//             return self.mantissa_biased() as i64 - 1023i64;
//         }

//         fn floor_log2(self) -> f64 {
//             if self <= 0f64 || !self.is_finite() {
//                 if self == f64::INFINITY || self.is_nan() {
//                     return self;
//                 }
//                 return self.make_nan();
//             }
//             return self.mantissa() as f64 - 1023f64;
//         }

//         fn fast_log2_unchecked(self) -> f64 {
//             let bits = self.to_bits();
//             let int_part1m = (bits >> 52 & 0b11111111111) as f64 - 1024f64;
//             let frac_part1p = f64::from_bits(
//                 bits & 0b0000000000001111111111111111111111111111111111111111111111111111u64  //sets the mantissa      
//                                                 |        0b0011111111110000000000000000000000000000000000000000000000000000u64,
//             ); //sets the exponent
//             return int_part1m + frac_part1p;
//         }

//         fn fast_log2(self) -> f64 {
//             if self <= 0f64 || !self.is_finite() {
//                 if self == f64::INFINITY || self.is_nan() {
//                     return self;
//                 }
//                 return make_nan(self);
//             }
//             return self.fast_log2_unchecked();
//         }

//         fn fast_log2_frac_unchecked(self) -> f64 {
//             return f64::from_bits(
//                 self.to_bits()
//                 & 0b0000000000001111111111111111111111111111111111111111111111111111u64  //sets the mantissa      
//                 | 0b0011111111110000000000000000000000000000000000000000000000000000u64, //sets the exponent
//             ) - 1f64;
//         }

//         fn fast_log2_frac(self) -> f64 {
//             if self <= 0f64 || !self.is_finite() {
//                 return make_nan(self);
//             }
//             return self.fast_log2_frac_unchecked();
//         }

//         /**
//          * Returns a quiet NaN with the sign of self, and the payload the first 52 bits of self after the sign bit unless self is already NaN, then self.
//          * This allows for retrieving an slightly less precise version of self.
//          */
//         fn make_nan(self) -> f64 {
//             if self.is_nan() {
//                 return self;
//             }
//             return f64::from_bits(
//                 (self.to_bits() >> 12)
//                     | 0b0111111111110000000000000000000000000000000000000000000000000000u64 //makes it a NaN
//                 & 0b1111111111110111111111111111111111111111111111111111111111111111u64, //quiets the NaN
//             )
//             .copysign(self);
//         }
//     }

//     pub trait FloatTraits<Int> {
//         const MANTISSA_BIAS: Int;

//         fn mantissa_biased(self) -> Int;

//         fn mantissa(self) -> Int;

//         fn floor_log2(self) -> Self;

//         fn fast_log2_unchecked(self) -> Self;

//         fn fast_log2(self) -> Self;

//         fn fast_log2_frac_unchecked(self) -> Self;

//         fn fast_log2_frac(self) -> Self;

//         /**
//          * Returns a quiet NaN with the sign of self, and the payload the first bits of self after the sign bit unless self is already NaN, then self.
//          * This allows for retrieving an slightly less precise version of self.
//          */
//         fn make_nan(self) -> Self;
//     }

//     pub trait RefAddition where for<'a> &'a Self: Add<&'a Self, Output = &'a Self> + AddAssign<&'a Self>, {}

//     pub trait RefSubtraction where for<'a> &'a Self: Sub<&'a Self, Output = Self> + SubAssign<&'a Self>, {}

//     pub trait RefMultiplication where for<'a> &'a Self: Mul<&'a Self, Output = Self> + MulAssign<&'a Self>, {}

//     pub trait RefDivision where for<'a> &'a Self: Div<&'a Self, Output = Self> + DivAssign<&'a Self>, {}

//     pub trait RefFourFunction where for<'a> &'a Self: RefAddition + RefMultiplication + RefSubtraction + RefDivision + Neg, {}

//     pub trait ExpLogRoot {
//         const E: Self;
//         const LN_2: Self;
//         const LN_10: Self;

//         ///e^self
//         fn exp(self) -> Self;

//         ///self ^ power
//         fn pow(self, power: Self);

//         ///2 ^ self
//         fn pow2(self);

//         ///√x
//         fn sqrt(self);

//         ///∛x
//         fn cert(self);

//         ///x^(1/n)
//         fn nthroot(self, n: Self);

//         fn floor_log2(self) -> Self;

//         fn log2(self) -> Self;
//         fn ln(self) -> Self;
//         fn log10(self) -> Self;
//     }

//     pub trait RefScientific
//     where
//         for<'a> &'a Self: ExpLogRoot + RefFourFunction,
//     {
//     }

//     pub trait NonFinite {
//         fn is_nan() -> bool;
//         fn is_finite() -> bool;
//         fn is_infinite() -> bool;
//         fn is_pos_infinity() -> bool;
//         fn is_neg_infinity() -> bool;
//         const INFINITY: Self;
//         const NEG_INFINITY: Self;
//         const NAN: Self;
//     }

//     pub trait traitname
//     where
//         for<'a> &'a Self: NonFinite + RefScientific,
//     {
//     }
//     //const IMPLEMENTS_COMPLEX: bool;d
//     // pub fn w0<T: traitname>(x: T) -> T {
//     //     if x.is_nan() || x == f64::INFINITY || x == 0f64 {
//     //         return x;
//     //     }
//     //     if x < -1f64/E {
//     //         return x.make_nan();
//     //     } else if x == -1f64/E {
//     //         return -1f64;
//     //     } else if x < E {
//     //         let mut prevs = if x < 0f64 {
//     //                 PreviousValsRecursive::new_in_range_with_1st(-1f64, -0f64, E*x)
//     //             } else if x <= 1.58f64 {
//     //                 PreviousValsRecursive::new_in_range_with_1st(0f64, 0.75f64, x/2f64)
//     //             } else {
//     //                 PreviousValsRecursive::new_in_range_with_1st(0.74f64, 1f64, (x/(x.mantissa() as f64*consts::LN_2)).fast_log2_unchecked()*consts::LN_2)
//     //             };
//     //         while prevs.ans.is_none() {
//     //             //println!("{}", prevs);
//     //             prevs.push(x/prevs.prev_value().exp());
//     //             //println!("{}",prevs);
//     //         }
//     //         //println!("{}",prevs);
//     //         if let Some(ans) = prevs.ans {return ans}
//     //         panic!();

//     //     } else if x == E {
//     //         return 1f64;
//     //     }
//     //     let mut prevs = if x < 5f64 {
//     //             PreviousValsRecursive::new_in_range_with_1st(0.74f64, 1.33f64, x/(2f64*E) + 0.5f64)
//     //         } else {
//     //             let guess = (x/(x.mantissa() as f64*consts::LN_2)).fast_log2_unchecked()*consts::LN_2;
//     //             PreviousValsRecursive::new_in_range_with_1st(1.32f64,if x<5.6 {1.4} else {guess}, guess)
//     //         };
//     //     while prevs.ans.is_none() {
//     //         prevs.push((x/prevs.prev_value()).ln());
//     //         //println!("{}",prevs);
//     //     }
//     //     //println!("{}",prevs);
//     //     if let Some(ans) = prevs.ans {return ans}
//     //     panic!();
//     // }
    #[derive(Debug)]
    pub struct TriplesArray {
        pub arr: Vec<Vec<Option<usize>>>,
        pub pointer: (usize, usize),
    }

    impl TriplesArray {
        
        pub fn len(&self) -> usize {
            self.arr.len()
        }
        pub fn omicron(&self) -> usize {
            self.len()
        }

        pub fn set_triple(&mut self, mut triple: (usize, usize, usize)) {
            if triple.0 < triple.1 {
                std::mem::swap(&mut triple.0, &mut triple.1);
            }
            if triple.1 < triple.2 {
                std::mem::swap(&mut triple.1, &mut triple.2);
            }
            if triple.0 < triple.1 {
                std::mem::swap(&mut triple.0, &mut triple.1);
            }
            // println!("setting: {:?}",triple);
            self.arr[triple.0][triple.1] = Some(triple.2);
            self.arr[triple.0][triple.2] = Some(triple.1);
            self.arr[triple.1][triple.2] = Some(triple.0);
        }
        
        pub fn remove_triple(&mut self, mut triple: (usize, usize, usize)) {
            if triple.0 < triple.1 {
                std::mem::swap(&mut triple.0, &mut triple.1);
            }
            if triple.1 < triple.2 {
                std::mem::swap(&mut triple.1, &mut triple.2);
            }
            if triple.0 < triple.1 {
                std::mem::swap(&mut triple.0, &mut triple.1);
            }

            self.arr[triple.0][triple.1] = None;
            self.arr[triple.0][triple.2] = None;
            self.arr[triple.1][triple.2] = None;
        }

        pub fn unset(&mut self, pointer: Option<(usize, usize)>) {
            let Some(e) = self.get(pointer) else {return}; 
            let p: (usize, usize) = pointer.unwrap_or(self.pointer);
            self.remove_triple((e, p.1, p.0));
        }

        pub fn test_quick(omicron: usize) -> Result<bool, ()> {
            if omicron <= 3 {return Ok(omicron==3)};
            if (omicron % 6)*((omicron-1) % 6) % 6  > 0 || (omicron & 1) == 0 {
                return Ok(false);
            }
            let mut trivials: Vec<usize> = vec![3];
            while trivials.len() > 0 {
                let cur = trivials.remove(0);
                if omicron == cur {
                    return Ok(true);
                }
                {
                    let new = cur.saturating_mul(cur-1).saturating_add(1);
                    if new <= omicron && new < usize::MAX {
                        trivials.b_insert_keep(new);
                    }
                }
                {
                    let new = cur.saturating_mul(cur);
                    if new <= omicron && new < usize::MAX {
                        trivials.b_insert_keep(new);
                    }
                }
                // println!("{:?}",trivials);
            }
            return Err(())
        }

        pub fn new(omicron: usize) -> TriplesArray {
            let mut arr = Vec::with_capacity(omicron);
            let mut column = Vec::with_capacity(0);
            for _ in 0..omicron {
                arr.push(column.clone());
                column.push(None);
            }
            let mut this = TriplesArray { arr, pointer: (3,1)};
            
            for i in 1..(1+this.arr.len())/2 {
                this.set_triple((2*i,2*i-1,0));
            }
            // println!("omicron: {}",omicron);
            // println!("arr: {}", Self::reformat(format!("{:?}",this.arr)));
            return this;
        }

        /// Returns Some(true) if it is full and is non conflicting <br>
        /// Returns Some(false) if it there is a un-resolvable conflict <br>
        /// Returns None if more steps need to happen to determine the conclusion
        pub fn step(&mut self) -> Option<bool> {
            self.move_pointer();
            if !self.pointer_inbounds(None) {
                return Some(true);
            }
            if let Some(val) = self.min_valid_value_at(None) {
                self.set_triple((self.pointer.0, self.pointer.1, val));
                return None;
            };
            
            while self.pointer.1 > 0 {
                if self.is_first_in_triple(None) {
                    if let Some(val) = self.min_valid_value_at(None) {
                        self.unset(None);
                        self.set_triple((val, self.pointer.0, self.pointer.1));
                        return None;
                    };
                    self.unset(None);
                    self.decrement_pointer();
                    
                } else {
                    self.decrement_pointer();
                }
            };
            return Some(false);
        }

        pub fn move_pointer(&mut self) {
            
            while self.pointer_inbounds(None) && self.get(None).is_some() {
                // println!("move pointer: {:?}", self.pointer);
                if self.pointer.0 == self.len()-1 {
                    self.pointer.1 += 1;
                    self.pointer.0 = self.pointer.1 + 1;
                } else {
                    self.pointer.0 += 1;
                }
            }
        }

        pub fn decrement_pointer(&mut self) {
            if self.pointer.0 == self.pointer.1 + 1 {
                self.pointer.1 -= 1;
                self.pointer.0 = self.len()-2;
            } else {
                self.pointer.0 -= 1;
            }
        }

        pub fn is_first_in_triple(&self, pointer: Option<(usize, usize)>) -> bool {
            let p: (usize, usize) = pointer.unwrap_or(self.pointer);
            return self.get(pointer).is_some_and(|e|e>p.0 && e>p.1);
        }

        /// Defaults to the saved pointer if None is given
        pub fn pointer_inbounds(&self, pointer: Option<(usize, usize)>) -> bool {
            let p = pointer.unwrap_or(self.pointer);
            p.0 < self.len() && p.1 < p.0
        }
        
        /// Defaults to the saved pointer if None is given <br>
        /// Returns None if oob
        pub fn get(&self, pointer: Option<(usize, usize)>) -> Option<usize> {
            let p = pointer.unwrap_or(self.pointer);
            self.arr.get(p.0)
                .map(|c|
                    c.get(p.1)
                    .cloned()
                    .unwrap_or_default()
                ).unwrap_or_default()
        }

        /// Defaults to the saved pointer if None is given <br>
        /// if there is already a value at the pointer, it will return a value greater than that
        pub fn min_valid_value_at(&self, pointer: Option<(usize, usize)>) -> Option<usize> {
            if !self.pointer_inbounds(pointer) {return None};
            let p = pointer.unwrap_or(self.pointer);

            let mut min = self.get(pointer).map(|l|l+1).unwrap_or_default();
            if min == self.len() {return None};

            let mut invalids = Vec::with_capacity(p.1);
            
            for i in 0..p.0 { //checking column
                //println!("checking: {},{}",p.0,i);
                if let Some(invalid) = self.arr[p.0][i] {
                    if min <= invalid {
                        invalids.b_insert_keep(invalid);
                    }
                    if min <= i {
                        invalids.b_insert_keep(i);
                    }
                }
            }
            for i in p.1+1..self.len() { //checking row
                //println!("checking: {},{}",i,p.1);
                if let Some(invalid) = self.arr[i][p.1] {
                    if min <= invalid {
                        invalids.b_insert_keep(invalid);
                    }
                    if min <= i {
                        invalids.b_insert_keep(i);
                    }
                }
            }
            if min <= p.0 {
                invalids.b_insert_keep(p.0);
            }
            if min <= p.1 {
                invalids.b_insert_keep(p.1);
            }
            
            // println!("p: {p:?}");
            // println!("at p: {:?}", self.get(pointer));
            // println!("min: {min:?}");
            // println!("invalids: {invalids:?}");
            if invalids.len() == 0 {return Some(min)};
            for invalid in invalids {
                if min < invalid {
                    return Some(min)
                } else {
                    min = min.max(invalid + 1);
                }
            }
            // println!("min: {min:?}");

            if min == self.len() {
                None
            } else {
                Some(min)
            }
        }
        
        pub fn to_table(&self) -> String {
            let mut max = None;
            for x in self.arr.iter() {
                for y in x {
                    max = max.max(y.clone());
                }
            }
            if max.is_none() {
                return Self::reformat(format!("{:?}",self.arr));
            }
            let e_padding = format!("{max:?}").len()-6;
            let i_padding = format!("{}",self.len()).len();
            let mut string = "".to_string();
            for i in 0..self.len() {
                let i_str = format!("{i}");
                string += &(" ".repeat(i_padding-i_str.len())+&i_str + ":  ");
                for ii in 0..i {
                    let e_str = match self.arr[i][ii] {
                        None => "_",
                        Some(e) => &format!("{e}"),
                    };
                    if ii < i-1 {
                        string += &(" ".repeat(e_padding-e_str.len())+&e_str + ", ");
                    } else {
                        string += &(" ".repeat(e_padding-e_str.len())+&e_str);
                    }
                }
                string += "\n"
            }
            string += &(".".repeat(i_padding+2+e_padding)+"0");
            for e in 1..self.len() - 1 {
                let e_str = &format!("{e}");
                string += &(".".repeat(e_padding-e_str.len()+2)+&e_str);
            }
            return string;
        }  

        pub fn reformat(string: String) -> String {
            string.replace("Some(", "").replace(")", "").replace("None", "_").replace("],","\n")
        }
    }

    pub enum Batches {
        Triple(TriplesArray),
    }

    impl Batches {
        pub fn test_quick(omicron: usize, phi: usize) -> Result<bool,()> {
            if omicron <= 1 {
                if phi == 0 {
                    return Ok(true);
                } else {
                    return Ok(false);
                }
            }

            if phi <= 1 {return Ok(false)};
            if phi == 2 {return Ok(true)};
            if phi == 3 {return TriplesArray::test_quick(omicron)};
            if phi == omicron {return Ok(true)};   
            if (omicron - 1) % (phi - 1) > 0 {return Ok(false)};

            let m = phi*(phi-1);

            if omicron <= m {return Ok(false)};

            if (omicron % m)*((omicron-1) % m) % m > 0 {
                return Ok(false);
            }

            let mut trivials: Vec<usize> = vec![phi];
            while trivials.len() > 0 {
                let cur = trivials.remove(0);
                if omicron == cur {
                    return Ok(true);
                }
                {
                    let new = cur.saturating_mul(cur-1).saturating_add(1);
                    if new <= omicron && new < usize::MAX {
                        trivials.b_insert_keep(new);
                    }
                }
                {
                    let new = cur.saturating_mul(cur);
                    if new <= omicron && new < usize::MAX {
                        trivials.b_insert_keep(new);
                    }
                }
            }
            return Err(())
        }
        pub fn test_medium(omicron: usize, phi: usize) -> Result<bool, ()> {
            let cur = Self::test_quick(omicron, phi);
            if phi != 3 || cur != Err(()) {return cur};
            // println!("omicron: {omicron}");
            let mut arr = TriplesArray::new(omicron);
            loop {
                match arr.step() {
                    None => (),
                    Some(ans) => return Ok(ans),
                }
            }
        }
    }
}
// use std::ops::{Add, AddAssign, DerefMut};
// pub enum Duration {
//     Permanent,
//     Temporary { duration: u8 },
// }

// impl Duration {
//     ///Returns true if the duration is not 0
//     pub fn decrement(&mut self) -> bool {
//         match self {
//             Self::Permanent => true,
//             Self::Temporary { duration } => {
//                 if *duration == 0 {
//                     return false;
//                 }
//                 *duration -= 1;
//                 return *duration > 0;
//             }
//         }
//     }

//     pub fn increment(&mut self) {
//         match self {
//             Self::Permanent => (),
//             Self::Temporary { duration } => *duration += 1,
//         }
//     }

//     pub fn is_over(self) -> bool {
//         match self {
//             Self::Permanent => false,
//             Self::Temporary { duration } => {
//                 return duration == 0;
//             }
//         }
//     }
// }

// impl Add for Duration {
//     type Output = Self;
//     fn add(self, rhs: Duration) -> Self::Output {
//         match self {
//             Self::Permanent => Self::Permanent,
//             Self::Temporary { duration } => {
//                 let d = duration;
//                 match rhs {
//                     Self::Permanent => Self::Permanent,
//                     Self::Temporary { duration } => Self::Temporary { duration: duration+ d },
//                 }
//             }
//         }
//     }
// }

// impl AddAssign for Duration {
//     fn add_assign(&mut self, rhs: Self) {
//         match rhs {
//             Self::Permanent => *self = Self::Permanent,
//             Self::Temporary { duration } => {
//                 let d = duration;
//                 match self {
//                     Self::Permanent => return,
//                     Self::Temporary { duration } => {
//                         *duration += d;
//                     }
//                 }
//             }
//         }
//     }
// }

// impl AddAssign<u8> for Duration {
//     fn add_assign(&mut self, rhs: u8) {
//         match self {
//             Self::Permanent => return,
//             Self::Temporary { duration } => {
//                 *duration += rhs;
//             }
//         }
//     }
// }

// impl Add<u8> for Duration {
//     type Output = Duration;
//     fn add(self, rhs: u8) -> Self::Output {
//         match self {
//             Self::Permanent => Self::Permanent,
//             Self::Temporary { duration } =>  Self::Temporary {duration: duration + rhs}
//         }
//     }
// }


use math::*;
// fn log_audit(x: f64) {
//     println!("                 x: {}", x);
//     println!("   mantissa biased: {}", x.mantissa_biased());
//     println!("          mantissa: {}", x.mantissa());
//     println!("             floor: {}", x.floor_log2());
//     println!("              fast: {}", x.fast_log2());
//     println!("     fast no check: {}", x.fast_log2_unchecked());
//     println!("         fast mod1: {}", x.fast_log2_frac());
//     println!("fast mod1 no check: {}", x.fast_log2_frac_unchecked());
//     println!("         canonical: {}", x.abs().log2());
// }
fn main() {
    println!("\nrunning\n");
    
    // for i in 13..14 {
    //     if 
    //         math::TriplesArray::test_quick(i).is_err() || 
    //         math::TriplesArray::test_quick(i).is_ok_and(|b|b) 
    //     {
    //         let mut arr = TriplesArray::new(i);
    //         println!("i: {}",i);
    //         loop {
    //             match arr.step() {
    //                 None => {
    //                     // if i>=5 {println!("{}", TriplesArray::reformat(format!("{arr:?}")))};
    //                     // if i>=10 {println!("{}", arr.to_table())};
    //                 },
    //                 Some(true) => {
    //                     println!("{}", arr.to_table()); 
    //                     println!("Ok(true) at {}",i); 
    //                     break
    //                 },
    //                 Some(false) => {
    //                     println!("{}", arr.to_table()); 
    //                     println!("Ok(false) at {}",i); 
    //                     break
    //                 },
    //             }
    //         }
    //         println!();
    //     }
    // }
    
    
    for i in 2..22 {
        print!("{}:  \t",i);
        // print!("2, ");
        let mut ii = 3;
        if i>2 {print!("2, ")};
        while ii*(ii-1) <= i {
            // match math::Batches::test_quick(i,ii) {
            match math::Batches::test_medium(i,ii) {
                Ok(true) => print!("{}, ",ii),
                Ok(false) => (),
                Err(()) => print!("?{}, ",ii),
            }
            ii += 1;
        }
        print!("{}",i);
        println!();
    }
}
