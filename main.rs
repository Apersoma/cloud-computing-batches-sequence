//#![allow(unused)]
#![allow(unused_imports)]
//use std::mem;
mod math {
    //use std::mem;
use std::fmt;
use std::cmp::Ordering;
use std::f64::consts;
use consts::E;
    struct PreviousValsRecursive {
        vals: Vec<f64>,
        ans: Option<f64>,
        min: f64,
        max: f64,
        prev: Option<f64>,
        prev_prev: Option<f64>,
    }

    impl std::fmt::Display for PreviousValsRecursive {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let vals = &self.vals;
            let mut k = "values: [".to_owned();
            if vals.len() == 0 {
                k = k.to_owned() + "]";
            } else {
                for i in 0..(vals.len()-1) {
                    k = k.to_owned() + &vals[i].to_string() + ",";
                }
                k = k.to_owned() + &vals[vals.len()-1].to_string() + "] ";
            }

            match self.ans {
                None => k = k.to_owned() + "ans: None",
                Some(x) => k = k.to_owned() + "ans: " + &x.to_string(),
            }
            
            match self.prev {
                None => k = k.to_owned() + " prev: None",
                Some(x) => k = k.to_owned() + " prev: " + &x.to_string(),
            }
            
            k = k.to_owned() + " interval: ["+ &self.min.to_string() + ", "+&self.max.to_string()+"]";
            write!(f,"{}",k)
        }
    }
        
    impl PreviousValsRecursive {

        fn push(&mut self, value: f64){
            if self.ans.is_some() {return}
            let prev_exists = self.prev.is_some();
            let min = &mut self.min;
            let max = &mut self.max;
            let vals = &mut self.vals;
            let mut set_prev = |value| {self.prev_prev = self.prev; self.prev = value};


            if vals.len()==0 && !prev_exists {
                if value < *min || value > *max{
                    set_prev(Some(value));
                } else {
                    vals.push(value);
                    set_prev(None);
                }
                return;
            }

            if vals.len()==1 {
                if prev_exists {
                    let oob = value < *min || value > *max;
                    if oob {
                        set_prev(Some(value));        
                    } else {
                        vals.push(value);
                        set_prev(None);
                    }
                    return;
                }
            }
            
            if prev_exists {
                if value < *min || value > *max{
                    set_prev(Some(value));
                    return;
                }
                vals.push(value);
                set_prev(None);
                
                if vals.len() >= 2{
                    for i in vals.len()-2..0{
                        if value == vals[i]{
                            self.ans = Some(value);
                            return;
                        }
                    }
                }
                
                return;
            }
            
            let prev = vals[vals.len()-1];
            if prev < value {
                *min = prev;
                if vals.len()>=2{
                    //println!("lbl 1");
                    let mut i = vals.len()-2;
                    loop {
                        if vals[i] < *min {
                            vals.remove(i);
                        }
                        if i == 0 {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                }
                if value > *max {
                    set_prev(Some(value));
                    return;
                } else {
                    vals.push(value);
                }
            } else if prev > value {
                *max = prev;
                if vals.len() >= 2 {
                    let mut i = vals.len() - 2;
                    loop {
                        if vals[i] > *max {
                            vals.remove(i);
                        }
                        if i == 0 {
                            break;
                        } else {
                            i -= 1;
                        }
                    }
                }
                if value < *min {
                    self.prev = Some(value);
                    return;
                } else {
                    vals.push(value);
                }
            } else {
                self.ans = Some(value);
                return;
            }
            
            if vals.len() >= 3{
                let mut i = vals.len() - 3;
                loop {
                    if value == vals[i] {
                        self.ans = Some(value);
                        return;
                    }
                    if i == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
        }
               
        fn new_in_range(min: f64, max: f64) -> PreviousValsRecursive {
            return PreviousValsRecursive {
                vals: Vec::with_capacity(3usize),
                ans: None,
                min: min,
                max: max,
                prev: None,
                prev_prev: None,
            }
        }
        
        fn new() -> PreviousValsRecursive {
            return Self::new_in_range(f64::NEG_INFINITY, f64::INFINITY);
        }
        
        fn new_in_range_with_1st(min: f64, max: f64, x: f64) -> PreviousValsRecursive{
            let mut k = Self::new_in_range(min, max);
            if min <= x && x <= max {
                k.push(x);
            } else {
                k.prev = Some(x);
            }
            return k;
        }

        fn new_with_1st(x: f64) -> PreviousValsRecursive{
            return Self::new_in_range_with_1st(f64::NEG_INFINITY, f64::INFINITY, x);
        }

        fn prev_value(&self) -> f64{
            return match self.prev {
                Some(prev) => prev,
                None => self.vals[self.vals.len()-1],
            }
        }

        fn prev_prev_value(&self) -> f64{
            if let Some(value) = self.prev_prev {
                return value;
            } else if self.prev.is_some() {
                return self.vals[self.vals.len()-1];
            } else {
                return self.vals[self.vals.len()-2];
            }
        }
        
        fn prev_option(&self) -> Option<&f64>{
            if self.prev.is_some() {
                return self.prev.as_ref();
            } else {
                return self.vals.get(self.vals.len()-1);
            }
        }

        fn prev_prev_option(&self) -> Option<&f64>{
            if  self.prev_prev.is_some() {
                return  self.prev_prev.as_ref();
            } else if self.prev.is_some() {
                return self.vals.get(self.vals.len()-1);
            } else {
                return self.vals.get(self.vals.len()-2);
            }
        }
    }

    pub fn fast_log2_unchecked(x: f64) -> f64{
        let bits = x.to_bits();
        let int_part1m = (bits >> 52 & 0b11111111111) as f64 - 1024f64;
        let frac_part1p = f64::from_bits(bits & 0b0000000000001111111111111111111111111111111111111111111111111111u64       //sets the mantissa      
                                            |        0b0011111111110000000000000000000000000000000000000000000000000000u64);//sets the exponent
        return int_part1m + frac_part1p;
    }

    pub fn floor_log2_unchecked(x: f64) -> i32 {
        return (x.to_bits() >> 52 & 0b11111111111) as i32 - 1023i32;
    }

    pub fn w0(x: f64) -> f64 {
        if x.is_nan() || x == f64::INFINITY{
            return x;
        }
        
        if x < -1f64/E {
            let mut bits = x.to_bits();
            bits = bits | 0b0111111111110000000000000000000000000000000000000000000000000000u64 //makes it a NaN
                        & 0b1111111111110111111111111111111111111111111111111111111111111111u64;//quiets the NaN
            return f64::from_bits(bits);
        } else if x == -1f64/E {
            return -1f64;
        } else if x == 0f64{
            return 0f64;
        } else if x < E {
            let mut prevs;
            
            if x < 0f64 {
                prevs = PreviousValsRecursive::new_in_range_with_1st(-1f64, -0f64, E*x);
            } else if x <= 1.58f64 {
                prevs = PreviousValsRecursive::new_in_range_with_1st(0f64, 0.75f64, x/2f64);
            } else {
                prevs = PreviousValsRecursive::new_in_range_with_1st(0.74f64, 1f64, fast_log2_unchecked(x/(floor_log2_unchecked(x) as f64*consts::LN_2))*consts::LN_2);
            }
            while prevs.ans.is_none() {
                //println!("{}", prevs);
                prevs.push(x/prevs.prev_value().exp());
            }
            
            if let Some(ans) = prevs.ans {return ans}
            panic!();
        } else if x == E {
            return 1f64;
        }
        let mut prevs;
        if x < 5f64 {
            prevs = PreviousValsRecursive::new_in_range_with_1st(0.74f64, 1.33f64, x/(2f64*E) + 0.5f64);
        } else {
            prevs = PreviousValsRecursive::new_in_range_with_1st(0f64, 1.32f64, x/2f64);
        }
        while prevs.ans.is_none() {
            prevs.push((x/prevs.prev_value()).ln());
        }
        
        if let Some(ans) = prevs.ans {return ans}
        panic!();
    }  
}
    fn main() {
        println!("{}", 1);
        // for i in -33..=33 {
        //     println!("        i: {}", i);
        //     println!("     fast: {}", fast_log2_unchecked(i as f64));
        //     println!("    floor: {}", floor_log2_unchecked(i as f64));
        //     println!("unrounded: {}", (i as f64).abs().log2());
        // }
        for i in 0..=20{
            println!(" i: {}", i);
            println!("w0: {}", math::w0(i as f64));
        }
    }
