use std::{collections::BTreeSet, fmt::Display};
#[allow(unused_imports)]
use std::mem;
use fxhash::FxBuildHasher;
use hashbrown::HashSet;
use crate::statics::*;

// #[allow(unused)]
// type StdHashSet<T> = std::collections::HashSet<T>;

#[macro_export]
macro_rules! generator_return {
    ($e:expr) => {
        #[cfg(all(debug_assertions, not(test)))]
        let batches = $e;
        #[cfg(all(debug_assertions, not(test)))]
        batches.audit().unwrap();
        // #[cfg(debug_assertions)]
        // println!("\n\nbatches: {batches}");
        // #[cfg(debug_assertions)]
        // batches.audit().unwrap();
        #[cfg(all(debug_assertions, not(test)))]
        return batches;
        #[cfg(not(all(debug_assertions, not(test))))]
        return $e;
    };
}

macro_rules! opt_generator_return {
    ($e:expr) => {

        let batches = $e;
        #[cfg(all(debug_assertions, not(test)))]
        batches.audit().unwrap();
        return Some(batches);
        
        /*
        #[cfg(all(debug_assertions, not(test)))] {
            let batches = $e;
            batches.audit().unwrap();
            return Some(batches);
        }
        
        #[cfg(not(all(debug_assertions, not(test))))]
        return Some($e);
        */
    };
}

#[macro_export]
macro_rules! insert_unique_hash {
    ($set:expr, $e:expr) => {
        #[cfg(debug_assertions)]
        assert!($set.insert($e));
        #[cfg(not(debug_assertions))]
        unsafe {$set.insert_unique_unchecked($e)}
    };
}

#[macro_export]
macro_rules! insert_unique_btree {
    ($set:expr, $e:expr) => {
        #[cfg(debug_assertions)]
        assert!($set.insert($e));
        #[cfg(not(debug_assertions))]
        $set.insert($e)
    };
}

type Passed = ();

type BatchHasher = FxBuildHasher;

#[inline(always)]
pub fn hashset<T>(capacity: usize) -> HashSet<T, BatchHasher>{
    HashSet::with_capacity_and_hasher(capacity, BatchHasher::default())
}

#[derive(Debug, Clone)]
pub struct Batches {
    pub omicron: u32,
    pub phi: u32,
    pub min: u32,
    pub max: u32,
    pub sets: HashSet<BTreeSet<u32>, BatchHasher>,
    // pub sets: StdHashSet<BTreeSet<u32>>,
}


impl Batches {
    #[expect(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.sets.len()
    }

    pub fn shift(&self, shift: i32) -> Batches {
        if shift == 0 {return self.clone()}
        let max = self.max.strict_add_signed(shift);
        let min = self.min.strict_add_signed(shift);
        
        let mut new_sets = hashset(self.len());
        if shift < 0 {
            let shift = shift.cast_unsigned();
            for mut set in self.sets.clone() {
                for e in set.clone().into_iter().rev() {
                    set.remove(&e);
                    insert_unique_btree!(set, e.wrapping_add(shift));
                }
                insert_unique_hash!(new_sets, set);
            }
        } else {
            let shift = shift.cast_unsigned();
            for mut set in self.sets.clone() {
                for e in set.clone() {
                    set.remove(&e);
                    insert_unique_btree!(set, e.wrapping_add(shift));
                }
                insert_unique_hash!(new_sets, set);
            }
        }

        generator_return!(Batches {
            sets: new_sets,
            min,
            max,
            omicron: self.omicron,
            phi: self.phi
        });
    }

    pub fn lambda(&self) -> u32 {
        (self.omicron - 1)/(self.phi - 1)
    }
    
    ///
    /// time complexity of `O(omicron²)`, and takes the longest when this is valid
    /// 
    pub fn audit(&self) -> Result<Passed, String> {
        if self.omicron - 1 + self.min != self.max {
            return Err(format!("Phi is too small. \
                phi = {}; omicron = {}; min = {}; max = {}", 
                self.phi, self.omicron, self.min, self.max
            ));
        }
        if self.phi <= 1 {
            return Err(format!(
                "Phi is too small. \
                phi = {}; omicron = {}; min = {}; max = {}", 
                self.phi, self.omicron, self.min, self.max
            ));
        }
        if self.omicron <= 1 {
            return Err(format!(
                "Omicron is too small. \
                phi = {}; omicron = {}; min = {}; max = {}", 
                self.phi, self.omicron, self.min, self.max
            ));
        }
        if self.phi != self.omicron && self.phi > self.omicron.max_phi_weak() {
            return Err(format!(
                "phi is too large. \
                phi = {}; omicron = {}",
                self.phi, self.omicron
            ));
        };
        if (self.omicron - 1) % (self.phi - 1) != 0 {
            return Err(format!(
                "Non-integral number of appearances of each element. \
                phi = {}; omicron = {}; min = {}; max = {}; computed lambda = {}", 
                self.phi, self.omicron, self.min, self.max, self.lambda()
            ));
        };
        // if test_quick(self.omicron, self.phi).is_ok_and(|v|!v) {
        //     panic!();
        // }


        let p = self.phi*(self.phi - 1);
        let o = self.omicron*(self.omicron-1);
        if o % p != 0 {
            return Err(format!(
                "Correct number of batches is non-integral for the given phi and omicron. \
                phi = {}; omicron = {}; min = {}; max = {}; expected len: {}; actual len: {}", 
                self.phi, self.omicron, self.min, self.max, (o as f64)/(p as f64), self.sets.len()
            ));
        }
        if self.sets.len() != (o / p) as usize {
            return Err(format!(
                "The number of batches is incorrect. \
                phi = {}; omicron = {}; min = {}; max = {}; expected len: {}; actual len: {}", 
                self.phi, self.omicron, self.min, self.max, o/p, self.sets.len()
            ))
        }
        
        if self.phi == 2 {
            for set in self.sets.iter() {
                if set.len() != 2 {
                    return Err(format!(
                        "Set is not size phi. set = {set:?}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ));
                }
                if let Some(set_min) = set.first() && *set_min < self.min {
                    return Err(format!(
                        "Set contains number below minimum value. set.min = {set_min}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ));
                }
                if let Some(set_max) = set.last() && *set_max > self.max {
                    return Err(format!(
                        "Set contains number above maximum value. set.max = {set_max}; \
                        min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                        self.min, self.max, self.phi, self.omicron
                    ));
                }
            }
            return Ok(())
        }
        if self.len() == 1 {
            let set = self.sets.iter().next().unwrap();
            if set.len() != self.phi as usize {
                return Err(format!(
                    "Set contains number. set = {set:?}; \
                    min = {}; max = {}; phi = {}; omicron = {}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
            if let Some(set_min) = set.first() && *set_min < self.min {
                return Err(format!(
                    "Set contains number below minimum value. set.min = {set_min}; \
                    min = {}; max = {}; phi = {}; omicron = {}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
            if let Some(set_max) = set.last() && *set_max > self.max {
                return Err(format!(
                    "Set contains number above maximum value. set.max = {set_max}; \
                    min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
        }
        
        let pair_count = ((self.omicron as usize -1)*self.omicron as usize)/2;
        // let mut pairs: HashSet<(u32, u32)> = HashSet::with_capacity(pair_count);
        // let mut pairs: StdHashSet<(u32, u32)> = StdHashSet::with_capacity(pair_count);
        let mut pairs: HashSet<(u32, u32), FxBuildHasher> = HashSet::with_capacity_and_hasher(pair_count, FxBuildHasher::default());

        // O(omicron²)
        // O(omicron²/phi)
        for set in self.sets.iter() {
            if set.len() != self.phi as usize {
                return Err(format!(
                    "Set is not size phi. set = {set:?}; \
                    min = {}; max = {}; phi = {}; omicron = {}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
            if let Some(set_min) = set.first() && *set_min < self.min {
                return Err(format!(
                    "Set contains number below minimum value. set.min = {set_min}; \
                    min = {}; max = {}; phi = {}; omicron = {}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
            if let Some(set_max) = set.last() && *set_max > self.max {
                return Err(format!(
                    "Set contains number above maximum value. set.max = {set_max}; \
                    min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                    self.min, self.max, self.phi, self.omicron
                ));
            }
            let mut elements = set.iter();
            while let Some(&x) = elements.next() {
                for &y in elements.clone() {
                    if !pairs.insert((x, y)) {
                         return Err(format!(
                            "Pair appears at least twice. pair = ({x}, {y}); \
                            min = {}; max = {}; phi = {}; omicron = {}", 
                            self.min, self.max, self.phi, self.omicron
                        ));
                    }
                }
            }
        }
        // the following could never have been reached because
        // that'd imply that either there was a duplicate pair or the size of one of the sets is
        // both of which will have already returned by now
        //
        // if pairs.len() != pair_count {
        //     return Err(format!(
        //         "Some pair never appears. \
        //         min = {}; max = {}; phi = {}; omicron = {}", 
        //         self.min, self.max, self.phi, self.omicron
        //     ));
        // }
        Ok(())
    }
    // pub fn generate()

    pub fn phi_equals_omicron(phi: u32, offset: u32) -> Batches {
        assert!(phi > 1);
        generator_return!(Batches { 
            omicron: phi,
            phi,
            min: offset,
            max: phi-1+offset, 
            sets: [BTreeSet::from_iter(offset..offset+phi)].into_iter().collect() 
        });
    }

    pub fn phi_2_n_phi_p_1(phi: u32, offset: u32) -> Option<Batches> {
        let phi_n1 = phi-1;
        let omicron = phi*(phi_n1)+1;
        if !phi_n1.is_prime() {
            if phi == 2 {
                opt_generator_return!(Batches { 
                    omicron,
                    phi, 
                    min: offset, 
                    max: omicron-1+offset,
                    sets: [
                        BTreeSet::from([offset, 1+offset]), 
                        BTreeSet::from([offset, 2+offset]),
                        BTreeSet::from([1+offset, 2+offset])
                    ].into_iter().collect()
                });
            }
            return None;
        }
        let mut sets = hashset(omicron as usize);
        let indices_to_base_value = |row: u32, column: u32| offset+row*phi_n1+column;

        for i in 0..phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(i,ii));
            }
            insert_unique_hash!(sets, set);
        }
        for i in 1..phi_n1 {
            for ii in 1..phi {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi_n1)+1,iii));
                }
                insert_unique_hash!(sets, set);
            }
        }
        for i in 1..phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, phi-1+offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(ii,i));
            }
            insert_unique_hash!(sets, set);
        }
        
        opt_generator_return!(Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets,
        });
    }

    pub fn phi_2(phi: u32, offset: u32) -> Option<Batches> {
        let omicron = phi*phi;
        if !phi.is_prime() {
            return None;
        }

        let mut sets = hashset(omicron as usize + phi as usize);
        
        let phi_n1 = phi-1;

        let indices_to_base_value = |row: u32, column: u32| offset+row*phi_n1+column;

        for i in 0..=phi {
            // if i == 1 {println!()};
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(i,ii));
            }
            // println!("{set:?}");
            insert_unique_hash!(sets, set);
        }

        for i in 1..phi {
            for ii in 1..=phi {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi)+1,iii));
                }
                insert_unique_hash!(sets, set);
            }
        }

        for i in 1..phi { 
            let mut set = BTreeSet::new();
            for ii in 1..=phi {
                insert_unique_btree!(set, indices_to_base_value(ii, i));
            }
            // println!("{set:?}");
            insert_unique_hash!(sets, set);
        }

        opt_generator_return!(Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets,
        });
    }

    /// Creates a net set with the same phi and an omicron that is this omicron times phi
    pub fn phi_x_omicron(&self) -> Batches {
        let mut sets = self.sets.clone();
        sets.reserve((self.phi as usize - 1 + self.omicron as usize)*self.omicron as usize);
        
        for i in 1..self.phi {
            let offset = i*self.omicron;
            // debug_assert!(self.min + offset > self.max);
            for og_set in self.sets.iter() {
                // insert_unique_btree!(sets, BTreeSet::from_iter(og_set.iter().map(|e|e+offset)));
                insert_unique_hash!(sets, BTreeSet::from_iter(og_set.iter().map(|e|e+offset)));
            }
        }

        for i in 0..self.omicron {
            for ii in 0..self.omicron {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, self.min + i);
                for iii in 1..self.phi {
                    insert_unique_btree!(set, self.min + self.omicron*iii + ((i*iii+ii) % self.omicron));
                }
                insert_unique_hash!(sets, set);
            }
        } 

        generator_return!(Batches {
            omicron: self.omicron * self.phi,
            phi: self.phi,
            min: self.min,
            max: self.max + (self.phi-1)*self.omicron,
            sets,
        });
    } 
}

impl Display for Batches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::with_capacity(
            ((self.omicron as usize*(self.omicron as usize-1))/(self.phi as usize*(self.phi as usize-1)))*(3+self.phi as usize*3)
        );
        for set in self.sets.iter().collect::<BTreeSet<_>>() {
            string.push_str(&format!("{set:?}\n"));
        }
        string.pop();
        f.write_str(&string)
    }
}