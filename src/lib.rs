extern crate next_prime;

use next_prime::next_prime;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A stack data structure with expected amortized O(1) lookup.
/// The interface is that of a stack, with an added "contains"
/// method for lookups. The implementation relies on a hash table
/// with (quadratic) open addressing. The index of the top of the
/// stack in the table is kept at all times, and the entries in the
/// table contain a key and an index to the value below them in the stack.
pub struct StackSet<T: Copy + Hash + PartialEq> {
    top_index: Option<usize>,
    count: usize,
    capacity: usize,
    table: Vec<Option<(T, Option<usize>)>>,
}

const INITIAL_CAPACITY: usize = 11;

impl<T: Copy + Hash + PartialEq> StackSet<T> {
    pub fn new() -> StackSet<T> {
        StackSet {
            top_index: None,
            count: 0,
            capacity: INITIAL_CAPACITY,
            table: (0..INITIAL_CAPACITY).map(|_| None).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top_index == None
    }

    pub fn top(&self) -> Option<T> {
        if self.top_index == None {
            None
        } else {
            Some(self.table[self.top_index.unwrap()].unwrap().0)
        }
    }

    pub fn pop(&mut self) -> T {
        if self.top_index == None {
            panic!("Tried to pop from empty StackSet!");
        }
        let entry = self.table[self.top_index.unwrap()].unwrap();
        let top = entry.0;
        self.table[self.top_index.unwrap()] = None;
        self.top_index = entry.1;
        top
    }

    pub fn push(&mut self, key: T) {
        self.push_at(self.hash(key), key);
    }

    pub fn contains(&self, key: T) -> bool {
        let mut offset = 1;
        let mut hash = self.hash(key);
        while self.table[hash] != None {
            if self.table[hash].unwrap().0 == key {
                return true;
            }
            hash += offset;
            offset += 2;
        }
        false
    }

    fn hash(&self, key: T) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish() as usize % self.table.len()
    }

    fn push_at(&mut self, mut hash: usize, key: T) {
        let mut offset = 1;
        while self.table[hash] != None {
            hash += offset;
            offset += 2;
        }
        self.table[hash] = Some((key, self.top_index));
        self.top_index = Some(hash);
        self.count += 1;
        if self.count >= self.capacity / 2 {
            self.resize()
        }
    }

    fn resize(&mut self) {
        let new_capacity = next_prime(self.capacity as u64 * 2);
        let extension: Vec<Option<(T, Option<usize>)>> = (self.capacity as u64
            ..new_capacity as u64)
            .map(|_| None)
            .collect();
        self.table.extend(extension);
        self.capacity = new_capacity as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::StackSet;

    #[test]
    fn new_stack_is_empty() {
        let stack: StackSet<usize> = StackSet::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn stack_is_not_empty_after_push() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.push(0);
        assert!(!stack.is_empty());
    }

    #[test]
    fn stack_has_no_top_when_empty() {
        let stack: StackSet<usize> = StackSet::new();
        assert_eq!(stack.top(), None);
    }

    #[test]
    fn top_returns_last_element_that_was_added() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.push(0);
        assert_eq!(stack.top(), Some(0));
        stack.push(1);
        assert_eq!(stack.top(), Some(1));
    }

    #[test]
    fn pop_returns_and_removes_top_element() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.push(0);
        stack.push(1);
        assert_eq!(stack.pop(), 1);
        assert_eq!(stack.pop(), 0);
    }

    #[test]
    #[should_panic]
    fn cannot_pop_from_empty_stack() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.pop();
    }

    #[test]
    fn empty_stack_contains_nothing() {
        let stack: StackSet<usize> = StackSet::new();
        assert!(!stack.contains(0));
    }

    #[test]
    fn stack_contains_element_that_was_added() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.push(0);
        assert!(stack.contains(0));
    }

    #[test]
    fn stack_does_not_contain_element_that_was_popped() {
        let mut stack: StackSet<usize> = StackSet::new();
        stack.push(0);
        stack.pop();
        assert!(!stack.contains(0));
    }

}
