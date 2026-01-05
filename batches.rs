use std::thread;
use std::sync::mpsc::channel;
use std::fmt::{Debug, Display};
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::mem;
use rustc_hash::FxBuildHasher;
pub use rustc_hash::FxBuildHasher as BatchHasher;
use hashbrown::HashSet;
use crate::{Int, signed, signed_unsigned::*, statics::*};

// #[allow(unused)]
// type StdHashSet<T> = std::collections::HashSet<T>;

#[macro_export]
macro_rules! generator_return {
    ($e:expr) => {
        let batches = $e;
        #[cfg(all(debug_assertions, not(test)))]
        batches.audit().unwrap();
        return batches;
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
        #[cfg(all(debug_assertions, not(feature = "fast-insertions")))]
        assert!($set.insert($e));
        #[cfg(not(all(debug_assertions, not(feature = "fast-insertions"))))]
        unsafe {$set.insert_unique_unchecked($e)};
        // #[cfg(not(all(debug_assertions, not(feature = "fast-insertions"))))]
        // panic!()
    };
}

#[macro_export]
macro_rules! insert_unique_btree {
    ($set:expr, $e:expr) => {
        #[cfg(all(debug_assertions, not(feature = "fast-insertions")))]
        assert!($set.insert($e));
        #[cfg(not(all(debug_assertions, not(feature = "fast-insertions"))))]
        $set.insert($e)
    };
}

macro_rules! send {
    ($s:expr, $v:expr) => {
        #[cfg(debug_assertions)]
        $s.send($v).unwrap();
        #[cfg(not(debug_assertions))]
        #[expect(unused_must_use)]
        $s.send($v);
    };
}

type Passed = ();

// type BatchHasher = FxBuildHasher;

#[inline(always)]
pub fn hashset<T>(capacity: usize) -> HashSet<T, BatchHasher>{
    HashSet::with_capacity_and_hasher(capacity, BatchHasher)
}

#[derive(Debug, Clone)]
pub struct Batches {
    pub omicron: Int,
    pub phi: Int,
    pub min: Int,
    pub max: Int,
    pub sets: HashSet<BTreeSet<Int>, BatchHasher>,
    // pub sets: StdHashSet<BTreeSet<Int>>,
}

pub struct ValidationError {
    pub err: String,
    pub severe: bool
}

impl Batches {
    #[expect(clippy::len_without_is_empty)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.sets.len()
    }

    #[must_use]
    pub fn shift(&self, shift: signed!(Int)) -> Batches {
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

    #[must_use]
    pub fn lambda(&self) -> Int {
        (self.omicron - 1)/(self.phi - 1)
    }

    #[inline(always)]
    pub fn pairs(&self) -> usize {
        self.omicron as usize * (self.omicron as usize - 1) / 2
    }
    
    ///
    /// time complexity of `O(omicron²)`, and takes the longest when this is valid
    /// 
    pub fn audit(&self) -> Result<Passed, ValidationError> {
        if self.omicron - 1 + self.min != self.max {
            return Err(ValidationError {
                err: format!(
                    "Phi is too small. \
                    phi = {}; omicron = {}; min = {}; max = {}", 
                    self.phi, self.omicron, self.min, self.max
                ),
                severe: true
            });
        }
        if self.phi <= 1 {
            return Err(ValidationError {
                err: format!(
                    "Phi is too small. \
                    phi = {}; omicron = {}; min = {}; max = {}", 
                    self.phi, self.omicron, self.min, self.max
                ),
                severe: true
            });
        }
        if self.omicron <= 1 {
            return Err(ValidationError {
                err: format!(
                    "Omicron is too small. \
                    phi = {}; omicron = {}; min = {}; max = {}", 
                    self.phi, self.omicron, self.min, self.max
                ),
                severe: true
            });
        }
        if self.phi != self.omicron && self.phi > self.omicron.max_phi_weak() {
            return Err(ValidationError {
                err: format!(
                    "phi is too large. \
                    phi = {}; omicron = {}",
                    self.phi, self.omicron
                ),
                severe: true
            });
        };
        if (self.omicron - 1) % (self.phi - 1) != 0 {
            return Err(ValidationError {
                err: format!(
                    "Non-integral number of appearances of each element. \
                    phi = {}; omicron = {}; min = {}; max = {}; computed lambda = {}", 
                    self.phi, self.omicron, self.min, self.max, self.lambda()
                ),
                severe: true
            });
        };
        // if test_quick(self.omicron, self.phi).is_ok_and(|v|!v) {
        //     panic!();
        // }

        
        let p = self.phi*(self.phi - 1);
        let o = self.omicron*(self.omicron-1);
        if o % p != 0 {
            return Err(ValidationError {
                err: format!(
                    "Correct number of batches is non-integral for the given phi and omicron. \
                    phi = {}; omicron = {}; min = {}; max = {}; expected len: {}; actual len: {}", 
                    self.phi, self.omicron, self.min, self.max, (o as f64)/(p as f64), self.sets.len()
                ),
                severe: true
            });
        }
        if self.sets.len() != (o / p) as usize {
            return Err(ValidationError {
                err: format!(
                    "The number of batches is incorrect. \
                    phi = {}; omicron = {}; min = {}; max = {}; expected len: {}; actual len: {}", 
                    self.phi, self.omicron, self.min, self.max, o/p, self.sets.len()
                ),
                severe: true
            });
        }
        
        if self.phi == 2 {
            for set in self.sets.iter() {
                if set.len() != 2 {
                    return Err(ValidationError {
                            err: format!(
                            "Set is not size phi. set = {set:?}; \
                            min = {}; max = {}; phi = {}; omicron = {}", 
                            self.min, self.max, self.phi, self.omicron
                        ),
                        severe: true
                    });
                }
                if let Some(set_min) = set.first() && *set_min < self.min {
                    return Err(ValidationError {
                        err: format!(
                            "Set contains number below minimum value. set.min = {set_min}; \
                            min = {}; max = {}; phi = {}; omicron = {}", 
                            self.min, self.max, self.phi, self.omicron
                        ),
                        severe: true
                    });
                }
                if let Some(set_max) = set.last() && *set_max > self.max {
                    return Err(ValidationError {
                        err: format!(
                            "Set contains number above maximum value. set.max = {set_max}; \
                            min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                            self.min, self.max, self.phi, self.omicron
                        ),
                        severe: true
                    });
                }
            }
            return Ok(())
        }
        
        if self.len() == 1 {
            let set = self.sets.iter().next().unwrap();
            if set.len() != self.phi as usize {
                return Err(ValidationError {
                    err: format!(
                        "Set is not size phi. set = {set:?}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
            if let Some(set_min) = set.first() && *set_min < self.min {
                return Err(ValidationError {
                    err: format!(
                        "Set contains number below minimum value. set.min = {set_min}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
            if let Some(set_max) = set.last() && *set_max > self.max {
                return Err(ValidationError {
                    err: format!(
                        "Set contains number above maximum value. set.max = {set_max}; \
                        min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
        }
        let pair_count = ((self.omicron as usize -1)*self.omicron as usize)/2;
        // let mut pairs: HashSet<(Int, Int)> = HashSet::with_capacity(pair_count);
        // let mut pairs: StdHashSet<(Int, Int)> = StdHashSet::with_capacity(pair_count);
        let mut pairs: HashSet<(Int, Int), FxBuildHasher> = HashSet::with_capacity_and_hasher(pair_count, FxBuildHasher);

        // O(omicron²)
        // O(omicron²/phi)
        for set in self.sets.iter() {
            if set.len() != self.phi as usize {
                return Err(ValidationError {
                    err: format!(
                        "Set is not size phi. set = {set:?}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
            if let Some(set_min) = set.first() && *set_min < self.min {
                return Err(ValidationError {
                    err: format!(
                        "Set contains number below minimum value. set.min = {set_min}; \
                        min = {}; max = {}; phi = {}; omicron = {}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
            if let Some(set_max) = set.last() && *set_max > self.max {
                return Err(ValidationError {
                    err: format!(
                        "Set contains number above maximum value. set.max = {set_max}; \
                        min = {}; max = {}; phi = {}; omicron = {};  set = {set:?}", 
                        self.min, self.max, self.phi, self.omicron
                    ),
                    severe: true
                });
            }
            let mut elements = set.iter();
            while let Some(&x) = elements.next() {
                for &y in elements.clone() {
                    if !pairs.insert((x, y)) {
                        return Err(ValidationError {
                            err: format!(
                                "Pair appears at least twice. pair = ({x}, {y}); \
                                min = {}; max = {}; phi = {}; omicron = {}", 
                                self.min, self.max, self.phi, self.omicron
                            ),
                            severe: false
                        });
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

    #[must_use]
    pub fn phi_equals_omicron(phi: Int, offset: Int) -> Batches {
        assert!(phi > 1);
        generator_return!(Batches { 
            omicron: phi,
            phi,
            min: offset,
            max: phi-1+offset, 
            sets: [BTreeSet::from_iter(offset..offset+phi)].into_iter().collect() 
        });
    }

    /// omicron = phi^2 - phi + 1
    #[must_use]
    pub fn phi_2_n_phi_p_1(phi: Int, offset: Int) -> Option<Batches> {
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
        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

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

    #[must_use]
    pub fn par_phi_2(phi: Int, offset: Int) -> Option<Batches> {
        if !phi.is_prime() {
            return None;
        }

        let phi_n1 = phi-1;
        let omicron = phi*phi;

        let mut sets = hashset(omicron as usize + phi as usize);
        
        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        let (sender, receiver) = channel();
        
        let collector = thread::spawn(move || {
            for set in receiver.iter() {
                insert_unique_hash!(sets, set);
            }
            sets
        });
        
        let sender_0 = sender.clone();
        thread::spawn(move ||
            for i in 1..phi_n1 {
                for ii in 1..phi {
                    let mut set = BTreeSet::new();
                    insert_unique_btree!(set, offset+i);
                    for iii in 1..phi {
                        insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi_n1)+1,iii));
                    }
                    send!(sender_0, set);
                }
            }
        );
        let sender_0 = sender.clone();
        thread::spawn(move ||
            for i in 0..=phi {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset);
                for ii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(i,ii));
                }
                send!(sender_0, set);
            }
        );
        thread::spawn(move ||
            for i in 1..phi { 
                let mut set = BTreeSet::new();
                for ii in 1..=phi {
                    insert_unique_btree!(set, indices_to_base_value(ii, i));
                }
                send!(sender, set);
            }
        );

        opt_generator_return!(Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets: collector.join().unwrap(),
        });
    }

    /// omicron = phi^2
    #[must_use]
    pub fn phi_2(phi: Int, offset: Int) -> Option<Batches> {
        if !phi.is_prime() {
            return None;
        }

        let omicron = phi*phi;

        let mut sets = hashset(omicron as usize + phi as usize);
        
        let phi_n1 = phi-1;

        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        for i in 0..=phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(i,ii));
            }
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
    #[must_use]
    pub fn phi_x_omicron(&self) -> Batches {
        // if self.phi == 2 {return Batches::phi_is_2(self.omicron*2, self.min)}
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

    // pub fn phi_is_2(omicron: Int, offset: Int) -> Batches {
    //     assert_ne!(omicron, 1);
    //     let mut sets = hashset(omicron as usize * (omicron as usize - 1) / 2);
    //     let max = offset+omicron;
    //     for a in offset..max {
    //         sets.extend((a+1..max).map(|b|[a,b].into()));
    //     }
    //     Batches { 
    //         omicron,
    //         phi: 2, 
    //         min: offset, 
    //         max, 
    //         sets
    //     }   
    // }
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

impl Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.severe {
            f.write_fmt(format_args!("=== Severe Batch Generation Error ===\n{}", self.err))
        } else {
            f.write_str(&self.err)
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.severe {
            f.write_fmt(format_args!("=== Severe Batch Generation Error ===\n{}", self.err))
        } else {
            f.write_str(&self.err)
        }
    }
}