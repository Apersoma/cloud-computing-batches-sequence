use std::thread;
use std::sync::mpsc::channel;
use std::fmt::{Debug, Display};
use std::collections::BTreeSet;
#[allow(unused_imports)]
use std::mem;
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use rustc_hash::FxBuildHasher;
pub use rustc_hash::FxBuildHasher as BatchHasher;
use hashbrown::HashSet;
use rayon::iter::ParallelExtend;
use crate::Int;
use crate::statics::*;

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
    pub sets: Vec<BTreeSet<Int>>,
    // pub sets: StdHashSet<BTreeSet<Int>>,
}

pub struct ValidationError {
    pub err: String,
    pub severe: bool
}

impl Batches {
    #[expect(clippy::len_without_is_empty)]
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.sets.len()
    }

    pub fn shift(&mut self, shift: Int, increase: bool) {
        if shift == 0 {return};
        if increase {
            //if any will fail, this will so everything else can be unchecked
            self.max = self.max.strict_add(shift);
            self.min = unsafe {self.max.unchecked_add(shift)};
            self.sets.par_iter_mut().for_each(|s|
                for e in s.clone() {
                    s.remove(&e);
                    unsafe {s.insert(e.unchecked_add(shift))};
                }
            );
        } else {
            self.min = self.min.strict_sub(shift);
            self.max = unsafe {self.max.unchecked_sub(shift)};
            self.sets.par_iter_mut().for_each(|s|
                for e in s.clone() {
                    s.remove(&e);
                    unsafe {s.insert(e.unchecked_sub(shift))};
                }
            );
        }        
    }

    #[must_use]
    #[inline(always)]
    pub fn lambda(&self) -> Int {
        (self.omicron - 1)/(self.phi - 1)
    }

    #[inline]
    pub fn pairs(&self) -> usize {
        self.omicron as usize * (self.omicron as usize - 1) / 2
    }
    
    ///
    /// time complexity of `O(omicron²)`, and takes the longest when this is valid
    /// 
    #[inline(always)]
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
            let set = self.sets.first().unwrap();
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
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn phi_equals_omicron(phi: Int, offset: Int) -> Batches {
        assert!(phi > 1);
        generator_return!(Batches { 
            omicron: phi,
            phi,
            min: offset,
            max: phi-1+offset, 
            sets: vec![BTreeSet::from_iter(offset..offset+phi)]
        });
    }

    /// omicron = phi^2 - phi + 1
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn phi_2_n_phi_p_1(phi: Int, offset: Int) -> Option<Batches> {
        if !(phi-1).is_prime() {
            if phi == 2 {
                opt_generator_return!(Batches { 
                    omicron: 3,
                    phi, 
                    min: offset, 
                    max: 2+offset,
                    sets: [
                        BTreeSet::from([offset, 1+offset]), 
                        BTreeSet::from([offset, 2+offset]),
                        BTreeSet::from([1+offset, 2+offset])
                    ].into_iter().collect()
                });
            }
            return None;
        }
        if phi < 50 {
            opt_generator_return!(Self::sequential_phi_2_n_phi_p_1_unchecked(phi, offset));
        } else {
            opt_generator_return!(Self::parallel_phi_2_n_phi_p_1_unchecked(phi, offset));
        }
    }
    
    /// omicron = phi^2 - phi + 1
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn sequential_phi_2_n_phi_p_1(phi: Int, offset: Int) -> Option<Batches> {
        if !(phi-1).is_prime() {
            if phi == 2 {
                opt_generator_return!(Batches { 
                    omicron: 3,
                    phi, 
                    min: offset, 
                    max: 2+offset,
                    sets: [
                        BTreeSet::from([offset, 1+offset]), 
                        BTreeSet::from([offset, 2+offset]),
                        BTreeSet::from([1+offset, 2+offset])
                    ].into_iter().collect()
                });
            }
            return None;
        }
        opt_generator_return!(Self::sequential_phi_2_n_phi_p_1_unchecked(phi, offset));
    }

    /// omicron = phi^2 - phi + 1
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn parallel_phi_2_n_phi_p_1(phi: Int, offset: Int) -> Option<Batches> {
        if !(phi-1).is_prime() {
            if phi == 2 {
                opt_generator_return!(Batches { 
                    omicron: 3,
                    phi, 
                    min: offset, 
                    max: 2+offset,
                    sets: [
                        BTreeSet::from([offset, 1+offset]), 
                        BTreeSet::from([offset, 2+offset]),
                        BTreeSet::from([1+offset, 2+offset])
                    ].into_iter().collect()
                });
            }
            return None;
        }
        opt_generator_return!(Self::parallel_phi_2_n_phi_p_1_unchecked(phi, offset));
    }


    /// omicron = phi^2 - phi + 1
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn parallel_phi_2_n_phi_p_1_unchecked(phi: Int, offset: Int) -> Batches {
        let phi_n1 = phi-1;
        let omicron = phi*(phi_n1)+1;

        let mut sets = Vec::with_capacity(omicron as usize);
        
        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        let (sender, receiver) = channel();
        
        let collector = thread::spawn(move || {
            for set in receiver.iter() {
                sets.push(set);
            }
            sets
        });

        let sender_0 = sender.clone();
        thread::spawn(move ||
            for i in 0..phi {
                
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset);
                for ii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(i,ii));
                }
                send!(sender_0, set);
            }
        );
        
        for i in 1..phi_n1 {
            let sender_0 = sender.clone();
            thread::spawn(move ||
                for ii in 1..phi {
                    let mut set = BTreeSet::new();
                    insert_unique_btree!(set, offset+i);
                    for iii in 1..phi {
                        insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi_n1)+1,iii));
                    }
                    send!(sender_0, set);
                }
            );
        }

        for i in 1..phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, phi-1+offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(ii,i));
            }
            send!(sender, set);
        }
        drop(sender);
        Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets: collector.join().unwrap(),
        }
    }

    /// omicron = phi^2 - phi + 1
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn sequential_phi_2_n_phi_p_1_unchecked(phi: Int, offset: Int) -> Batches {
        let phi_n1 = phi-1;
        let omicron = phi*(phi_n1)+1;
        

        let mut sets = Vec::with_capacity(omicron as usize);
        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        for i in 0..phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(i,ii));
            }
            sets.push(set);
        }
        for i in 1..phi_n1 {
            for ii in 1..phi {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi_n1)+1,iii));
                }
                sets.push(set);
            }
        }
        for i in 1..phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, phi-1+offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(ii,i));
            }
            sets.push(set);
        }
        
        Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets,
        }
    }

    /// omicron = phi^2
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn phi_2(phi: Int, offset: Int) -> Option<Batches> {
        if !phi.is_prime() {
            return None;
        }

        if phi < 50 {
            opt_generator_return!(Self::sequential_phi_2_unchecked(phi, offset));
        } else {
            opt_generator_return!(Self::parallel_phi_2_unchecked(phi, offset));
        }
    }
    
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn parallel_phi_2(phi: Int, offset: Int) -> Option<Batches> {
        if !phi.is_prime() {
            return None;
        }
        opt_generator_return!(Self::parallel_phi_2_unchecked(phi, offset));
    }

    /// omicron = phi^2
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn parallel_phi_2_unchecked(phi: Int, offset: Int) -> Batches {
        let phi_n1 = phi-1;
        let omicron = phi*phi;

        let mut sets = Vec::with_capacity(omicron as usize + phi as usize);
        
        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        let (sender, receiver) = channel();
        
        let collector = thread::spawn(move || {
            for set in receiver.iter() {
                sets.push(set);
            }
            sets
        });
        
        for i in 1..phi_n1 {
            let sender_0 = sender.clone();
            thread::spawn(move ||
                for ii in 1..phi {
                    let mut set = BTreeSet::new();
                    insert_unique_btree!(set, offset+i);
                    for iii in 1..phi {
                        insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi_n1)+1,iii));
                    }
                    send!(sender_0, set);
                }
            );
        }

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
        for i in 1..phi { 
            let mut set = BTreeSet::new();
            for ii in 1..=phi {
                insert_unique_btree!(set, indices_to_base_value(ii, i));
            }
            send!(sender, set);
        }
        drop(sender);

        Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets: collector.join().unwrap(),
        }
    }

    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn sequential_phi_2(phi: Int, offset: Int) -> Option<Batches> {
        if !phi.is_prime() {
            return None;
        }
        opt_generator_return!(Self::sequential_phi_2_unchecked(phi, offset));
    }

    /// omicron = phi^2
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn sequential_phi_2_unchecked(phi: Int, offset: Int) -> Batches {
        let omicron = phi*phi;

        let mut sets = Vec::with_capacity(omicron as usize + phi as usize);
        
        let phi_n1 = phi-1;

        let indices_to_base_value = move |row: Int, column: Int| offset+row*phi_n1+column;

        for i in 0..=phi {
            let mut set = BTreeSet::new();
            insert_unique_btree!(set, offset);
            for ii in 1..phi {
                insert_unique_btree!(set, indices_to_base_value(i,ii));
            }
            sets.push(set);
        }

        for i in 1..phi {
            for ii in 1..=phi {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, offset+i);
                for iii in 1..phi {
                    insert_unique_btree!(set, indices_to_base_value(((ii+(iii-1)*(i) - 1)%phi)+1,iii));
                }
                sets.push(set);
            }
        }

        for i in 1..phi { 
            let mut set = BTreeSet::new();
            for ii in 1..=phi {
                insert_unique_btree!(set, indices_to_base_value(ii, i));
            }
            sets.push(set);
        }

        Batches {
            omicron,
            phi,
            min: offset,
            max: omicron - 1 + offset,
            sets,
        }
    }

    /// Creates a net set with the same phi and an omicron that is this omicron times phi
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn phi_x_omicron(&self) -> Batches {
        if self.phi == 2 {
            return Self::batches_of_pairs(self.omicron*2, self.min)
        }
        // if self.phi == 2 {return Batches::phi_is_2(self.omicron*2, self.min)}
        let mut sets = self.sets.clone();
        sets.reserve((self.phi as usize - 1 + self.omicron as usize)*self.omicron as usize);
        
        for i in 1..self.phi {
            let offset = i*self.omicron;
            // debug_assert!(self.min + offset > self.max);
            for og_set in self.sets.iter() {
                // insert_unique_btree!(sets, BTreeSet::from_iter(og_set.iter().map(|e|e+offset)));
                sets.push(BTreeSet::from_iter(og_set.iter().map(|e|e+offset)));
            }
        }

        for i in 0..self.omicron {
            for ii in 0..self.omicron {
                let mut set = BTreeSet::new();
                insert_unique_btree!(set, self.min + i);
                for iii in 1..self.phi {
                    insert_unique_btree!(set, self.min + self.omicron*iii + ((i*iii+ii) % self.omicron));
                }
                sets.push(set);
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

    /// is parallel but via rayon
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn batches_of_pairs(omicron: Int, offset: Int) -> Batches {
        let mut sets = Vec::with_capacity(omicron as usize *(omicron as usize - 1) / 2);
        sets.par_extend(
            (offset..omicron+offset)
            .into_par_iter()
            .flat_map(|x|(offset..x).into_par_iter().map(move |y|BTreeSet::from([x,y])))
        );
        generator_return!(Batches {
            omicron,
            phi: 2,
            min: offset,
            max: offset+omicron-1,
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