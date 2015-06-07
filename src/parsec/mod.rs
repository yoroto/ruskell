//use std::result::Result;
use std::vec::Vec;
use std::iter::{Iterator, FromIterator};

pub struct VecState<T> {
    index : usize,
    buffer: Vec<T>,
}

impl<A> FromIterator<A> for VecState<A> {
    fn from_iter<T>(iterator: T) -> Self where T:IntoIterator<Item=A> {
        VecState{
            index:0,
            buffer:Vec::from_iter(iterator),
        }
    }
}

impl<T:Clone> VecState<T> {
    pub fn current(&self) -> usize {
        self.index
    }
    pub fn seek_to(&mut self, to:usize) -> bool {
        if 0 as usize <= to && to < self.buffer.len() {
            self.index = to;
            true
        } else {
            false
        }
    }
    pub fn next(&mut self)->Option<T>{
        if 0 as usize <= self.index && self.index < self.buffer.len() {
            let item = &self.buffer[self.index];
            self.index += 1;
            Some(item).cloned()
        } else {
            None
        }
    }
    pub fn next_by(&mut self, pred:&Fn(&T)->bool)->Option<T>{
        if 0 as usize
         <= self.index && self.index < self.buffer.len() {
            let item = &self.buffer[self.index];
            if pred(item) {
                self.index += 1;
            }
            Some(item).cloned()
        } else {
            None
        }
    }
}
