

use std::{collections::BTreeSet, time::Instant};
#[allow(unused_imports)]
use hashbrown::HashSet;
// use std:w:collections::HashSet;

use crate::batches::*;
use crate::generator_return;
use crate::Int;
use crate::binary_collections::*;
use crate::push_element;
use crate::statics::*;

type Triple = (usize, usize, usize);
type Pointer = (usize, usize);

#[derive(Debug, Clone)]
pub struct TriplesArray {
    pub arr: Vec<Vec<Option<usize>>>,
    pub pointer: Pointer,
    pub mem: Vec<usize>,
}

macro_rules! pointer {
    ($p:expr, $s:expr) => {
        $p.unwrap_or($s.pointer)
    };
}

impl TriplesArray {
    pub const PHI: usize = 3;
    
    #[expect(clippy::len_without_is_empty, reason="always false")]
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.arr.len()
    }
    
    #[must_use]
    #[inline(always)]
    pub fn omicron(&self) -> usize {
        self.len()
    }

    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn batch_count(&self) -> usize {
        (self.omicron()*(self.omicron()-1))/6
    }
    
    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn sort_triple(mut triple: Triple) -> Triple {
        if triple.0 < triple.1 {
            std::mem::swap(&mut triple.0, &mut triple.1);
        }
        if triple.1 < triple.2 {
            std::mem::swap(&mut triple.1, &mut triple.2);
        }
        if triple.0 < triple.1 {
            std::mem::swap(&mut triple.0, &mut triple.1);
        }
        triple
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn set_triple(&mut self, mut triple: Triple) {
        triple = Self::sort_triple(triple);
        self.arr[triple.0][triple.1] = Some(triple.2);
        self.arr[triple.0][triple.2] = Some(triple.1);
        self.arr[triple.1][triple.2] = Some(triple.0);
    }
    
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn remove_triple(&mut self, mut triple: Triple) {
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

    #[inline]
    pub fn unset(&mut self, pointer: Option<Pointer>) {
        let Some(e) = self.get(pointer) else {return}; 
        let p: Pointer = pointer!(pointer, self);
        self.remove_triple((e, p.1, p.0));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn test(omicron: usize, status_updates: bool) -> bool {
        let cur = omicron.test_quick(3);
        if let Ok(ans) = cur {return ans};
        println!("{omicron} : {status_updates}");
        Self::generate_stupid(omicron, status_updates).is_ok()
    }

    /*
        does absolutely no fastpathing for easily generated batches
     */
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn generate_stupid(omicron: usize, status_updates: bool) -> Result<Self, Self> {
        let mut arr = Self::new(omicron);
        if omicron == 2 {return Err(arr)};
        // if omicron > 13 && omicron != 15 && omicron != 19 {
        //     println!("{}", arr.to_table());
        // }
        if arr.check(status_updates) {
            Ok(arr)
        } else {
            Err(arr)
        }
    }
    
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn check(&mut self, status_updates: bool) -> bool {
        let mut i = 1u32;
        let mut cur = Instant::now();
        loop {
            match self.step() {
                None => if status_updates {
                    i=(i+1)%(1<<25); 
                    if i==0 {
                        println!("{}", self.to_table());
                        println!("{:?}", cur.elapsed());
                        cur = Instant::now();
                    }
                },
                Some(ans) => {
                    // if status_updates {println!("{}", self.to_table())};
                    return ans;
                },
            }
        }
    }

    #[must_use]
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new(omicron: usize) -> TriplesArray {
        let mut arr = Vec::with_capacity(omicron);
        let mut column = Vec::with_capacity(0);
        for _ in 0..omicron {
            arr.push(column.clone());
            column.push(None);
        }
        let mut this = TriplesArray {arr, pointer: (3,1), mem: Vec::with_capacity(omicron.ilog2() as usize)};
        
        for i in 1..this.arr.len().div_ceil(2) {
            this.set_triple((2*i,2*i-1,0));
        }
        this
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn step(&mut self) -> Option<bool> {
        
        let prev_pointer = self.pointer;//@
        
        self.pointer_to_front();
        if !self.pointer_inbounds(None) {
            return Some(true);
        }
        if let Some(val) = self.min_valid_value() {
            self.set_triple((self.pointer.0, self.pointer.1, val));
            return None;
        };
        self.pointer = prev_pointer;//@
        if self.omicron() == 21 {println!("foo")};
        while self.pointer.1 > 0 {
            if self.is_first_in_triple(None) {
                if let Some(val) = self.min_valid_value() {
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
        Some(false)
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn pointer_to_front(&mut self) {
        while self.get(None).is_some() {
            if self.pointer.0 == self.len()-2 {
                self.pointer.1 += 1;
                self.pointer.0 = self.pointer.1 + 1 + (self.pointer.1&1);
            } else {
                self.pointer.0 += 1;
            }
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn decrement_pointer(&mut self) {
        if self.pointer.0 == self.pointer.1 + 1 + (self.pointer.1&1) {
            self.pointer.1 -= 1;
            self.pointer.0 = self.len()-2;
        } else {
            self.pointer.0 -= 1;
        }
    }

    #[must_use]
    #[inline]
    pub fn is_first_in_triple(&self, pointer: Option<Pointer>) -> bool {
        let p: Pointer = pointer!(pointer, self);
        self.get(pointer).is_some_and(|e|e>p.0 && e>p.1)
    }

    /// Defaults to the saved pointer if None is given
    #[must_use]
    #[inline]
    pub fn pointer_inbounds(&self, pointer: Option<Pointer>) -> bool {
        let p = pointer!(pointer, self);
        p.0 < self.len() && p.1 < p.0
    }
    
    /// Defaults to the saved pointer if None is given <br>
    /// Returns None if oob
    #[must_use]
    #[inline]
    pub fn get(&self, pointer: Option<Pointer>) -> Option<usize> {
        let p = pointer!(pointer, self);
        self.arr.get(p.0)
            .map(|c|
                c.get(p.1)
                .cloned()
                .unwrap_or_default()
            ).unwrap_or_default()
    }

    #[must_use]
    #[inline]
    pub fn get_triple(&self, pointer: Option<Pointer>) -> Option<Triple> {
        let p = pointer!(pointer, self);
        self.arr.get(p.0)
            .map(|c|
                c.get(p.1)
                .cloned()
                .unwrap_or_default()
            ).unwrap_or_default().map(|e|Self::sort_triple((e, p.0, p.1)))
    }

    /// Defaults to the saved pointer if None is given <br>
    /// if there is already a value at the pointer, it will return a value greater than that
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn min_valid_value(&mut self) -> Option<usize> {
        let mut min = (self.get(None).unwrap_or_default()+1).max(self.pointer.0+1).max(self.pointer.1+1);//@
        if min == self.len() {return None};

        let invalids = &mut self.mem;
        invalids.clear();   
        for i in 0..self.pointer.0 {
            if 
                let Some(invalid) = self.arr[self.pointer.0][i] &&
                min <= invalid
            {
                invalids.b_insert_keep(invalid);
            }
        }
        for i in (self.pointer.1+1)|1..self.arr.len() { //checking row
            if let Some(invalid) = self.arr[i][self.pointer.1] {
                if min <= invalid {
                    invalids.b_insert_keep(invalid);
                }
                if min <= i {
                    invalids.b_insert_keep(i);
                }
            }
        }

        if invalids.is_empty() {return Some(min)};
        for invalid in invalids {
            if min < *invalid {
                return Some(min)
            } else {
                min = min.max(*invalid + 1);
            }
        }

        if min == self.len() {
            None
        } else {
            Some(min)
        }
    }
    
    #[must_use]
    pub fn to_table(&self) -> String {
        let mut max = None;
        for x in self.arr.iter() {
            for y in x {
                max = max.max(*y);
            }
        }


        if max.is_none() {
            return format!("{:?}",self.arr)
                .replace("Some(", "")
                .replace("None", "_")
                .replace(")", "")
                .replace("],","\n");
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
                    string += &(" ".repeat(e_padding-e_str.len())+e_str + ", ");
                } else {
                    string += &(" ".repeat(e_padding-e_str.len())+e_str);
                }
            }
            string += "\n"
        }
        string += &(".".repeat(i_padding+2+e_padding)+"0");
        for e in 1..self.len() - 1 {
            let e_str = &format!("{e}");
            string += &(".".repeat(e_padding-e_str.len()+2)+e_str);
        }
        string
    } 

    #[must_use]
    pub fn to_subsets(&self, offset: usize) -> String {
        // either "{0, 0, 0}"
        // or     
        // or longer
        let mut string = String::with_capacity(((self.omicron()*(self.omicron()-1))/2) * 3);
        // let mut fmt_set = |set: String| set.replace("Some((", "{").replace("))", "}");
        for i in 1..self.len() {
            for ii in 0..i {
                if self.get(Some((i,ii))).is_none() {
                    string += "Undefined\n"
                } else if self.is_first_in_triple(Some((i,ii))) {
                    string += &(self.triple_set(Some((i,ii)), offset) +"\n");
                }
            }
        }
        string
    }

    #[must_use]
    #[inline]
    pub fn triple_set(&self, pointer: Option<Pointer>, offset: usize) -> String {
        let triple = self.get_triple(pointer).unwrap();
        format!(
            "{{{}, {}, {}}}",
            triple.0 + offset, 
            triple.1 + offset, 
            triple.2 + offset
        )
    }

    #[must_use]
    pub fn from_string(string: &str) -> Self {
        let mut split: Vec<&str> = string.trim().split('\n').collect();
        split.pop();
        let mut arr: Vec<Vec<Option<usize>>> = Vec::with_capacity(split.len());
        let c_start = 3+split[0].find(':').unwrap();
        arr.push(vec![]);
        split.remove(0);
        for c in split {
            let mut col = Vec::with_capacity(arr.len());
            for e in c[c_start..].split(',') {
                let e_trim = e.trim();
                match e_trim.parse::<usize>() {
                    Ok(val) => col.push(Some(val)),
                    Err(_) => col.push(None),
                }
            }
            arr.push(col);
        }
        Self {
            arr,
            mem: vec![],
            pointer: (3,1),
        }
    }
}

impl From<TriplesArray> for Batches {
    fn from(value: TriplesArray) -> Self {
        let omicron = value.omicron().try_into().unwrap();
        #[cfg(feature = "vec")]
        let mut sets = Vec::with_capacity(value.batch_count());
        #[cfg(not(feature = "vec"))]
        let mut sets = std::collections::LinkedList::new();
        
        for i in 1..value.len() {
            for ii in 0..i {
                if value.get(Some((i,ii))).is_none() {
                    panic!()
                } else if value.is_first_in_triple(Some((i,ii))) {
                    let triple = value.get_triple(Some((i,ii))).unwrap();
                    push_element!(sets, BTreeSet::from([triple.0 as Int, triple.1 as Int, triple.2 as Int]));
                }
            }
        }
        
        generator_return!(Batches { 
            omicron,
            phi: 3,
            min: 0, 
            max: omicron - 1,
            sets
        });
    }   
}

