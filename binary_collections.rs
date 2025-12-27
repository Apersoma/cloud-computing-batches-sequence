use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::Index;
use std::ops::IndexMut;

pub trait BinaryCollection<Element> where Element: PartialOrd {

    /// Returns the index of the greatest element less than or equal to e
    fn b_search_less(&self, e: &Element) -> usize;

    /// Returns the index of the least element greater than or equal to e
    fn b_search_greater(&self, e: &Element) -> usize;

    /// Returns the index of e, None if it is not in there
    fn b_search(&self, e: &Element) -> Option<usize>;

    fn b_contains(&self, e: &Element) -> bool;
}

pub trait BinaryResizableCollection<Element>: BinaryCollection<Element> where Element: PartialOrd  {
    fn b_insert_replace(&mut self, e: Element) -> Option<Element>;

    fn b_insert_keep(&mut self, e: Element) -> Option<&Element>;
    
    fn b_insert_before(&mut self, e: Element);

    fn b_insert_after(&mut self, e: Element);
}

macro_rules! binary_collection {
    () => {
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
            high
        }
        
        fn b_search_greater(&self, e: &E) -> usize {
            if self.is_empty() {return 0};
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
            low
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
            None
        }

        fn b_contains(&self, e: &E) -> bool {
            let mut low = 0;
            let mut high = self.len()-1;
        
            while low <= high {
                let mid = low + (high-low)/2;
                match self[mid].cmp(e) {
                    Ordering::Equal => return true,
                    Ordering::Less => low = mid+1,
                    Ordering::Greater => high = mid-1,
                }
            }
            false
        }
    };
}

impl<E> BinaryCollection<E> for Vec<E> where E: Ord+Sized {
    binary_collection!();
}

impl<E> BinaryResizableCollection<E> for Vec<E> where E: Ord+Sized {
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
        None
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
        None
    }

    fn b_insert_before(&mut self, e: E) {
        self.insert(self.b_search_greater(&e), e);
    }

    fn b_insert_after(&mut self, e: E) {
        self.insert(self.b_search_greater(&e).min(self.len()-1)+1, e);
    }
}

impl<E> BinaryCollection<E> for VecDeque<E> where E: Ord+Sized {
    binary_collection!();
}

impl<E> BinaryResizableCollection<E> for VecDeque<E> where E: Ord+Sized {
    fn b_insert_replace(&mut self, mut e: E) -> Option<E> {
        let i = self.b_search_greater(&e);
        if i==self.len() {
            self.push_back(e);
            return None;
        }
        
        let x = &mut self[i];
        if *x == e  {
            std::mem::swap(x, &mut e);
            return Some(e);
        }

        self.insert(i, e);
        None
    }

    fn b_insert_keep(&mut self, e: E) -> Option<&E> {
        let i = self.b_search_greater(&e);
        // println!("i:{}",i);
        if i==self.len() {
            self.push_back(e);
            return None;
        }
        
        if self[i] == e {
            return Some(&self[i]);
        }

        self.insert(i, e);
        None
    }
    
    fn b_insert_before(&mut self, e: E) {
        self.insert(self.b_search_greater(&e), e);
    }

    fn b_insert_after(&mut self, e: E) {
        self.insert(self.b_search_greater(&e).min(self.len()-1)+1, e);
    }
}

impl<E, const L: usize> BinaryCollection<E> for [E; L] where E: Ord+Sized {
    binary_collection!();
}


