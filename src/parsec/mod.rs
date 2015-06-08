mod atom;
use std::vec::Vec;
use std::sync::Arc;
use std::iter::{Iterator, FromIterator};

pub struct VecState<T> {
    index : usize,
    buffer: Vec<Arc<T>>,
}

impl<A> FromIterator<A> for VecState<A> {
    fn from_iter<T>(iterator: T) -> Self where T:IntoIterator<Item=A> {
        VecState{
            index:0,
            buffer:Vec::from_iter(iterator.into_iter().map(|x:A|Arc::new(x))),
        }
    }
}

pub trait State<T> {
    fn pos(&self)-> usize;
    fn seek_to(&mut self, usize)->bool;
    fn next(&mut self)->Option<Arc<T>>;
    fn next_by(&mut self, &Fn(Arc<T>)->bool)->Option<Arc<T>>;
}

impl<T> State<T> for VecState<T> {
    fn pos(&self) -> usize {
        self.index
    }
    fn seek_to(&mut self, to:usize) -> bool {
        if 0 as usize <= to && to < self.buffer.len() {
            self.index = to;
            true
        } else {
            false
        }
    }
    fn next(&mut self)->Option<Arc<T>>{
        if 0 as usize <= self.index && self.index < self.buffer.len() {
            let item = self.buffer[self.index].clone();
            self.index += 1;
            Some(item.clone())
        } else {
            None
        }
    }
    fn next_by(&mut self, pred:&Fn(Arc<T>)->bool)->Option<Arc<T>>{
        if 0 as usize
         <= self.index && self.index < self.buffer.len() {
            let item = self.buffer[self.index].clone();
            if pred(item.clone()) {
                self.index += 1;
            }
            Some(item.clone())
        } else {
            None
        }
    }
}

pub struct SimpleError {
    _at: usize,
    _message: String,

}
impl SimpleError {
    pub fn new(at:usize, message:String)->SimpleError{
        SimpleError{
            _at: at,
            _message: message,
        }
    }
}

pub trait Error {
    fn at(&self)->usize;
    fn message(&self)->&str;
}

impl Error for SimpleError {
    fn at(&self)->usize {
        self._at
    }
    fn message(&self)->&str {
        self._message.as_str()
    }
}
