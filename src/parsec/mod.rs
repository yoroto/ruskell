use std::vec::Vec;
use std::sync::Arc;
use std::iter::{Iterator, FromIterator};
use std::fmt::{Debug, Formatter};
use std::fmt;

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
    fn next_by(&mut self, &Fn(Arc<T>)->bool)->Status<T>;
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
    fn next_by(&mut self, pred:&Fn(Arc<T>)->bool)->Status<T>{
        if 0 as usize <= self.index && self.index < self.buffer.len() {
            let item = self.buffer[self.index].clone();
            if pred(item.clone()) {
                self.index += 1;
                Ok(item)
            } else {
                Err(SimpleError::new(self.index, String::from("predicate failed")))
            }
        } else {
            Err(SimpleError::new(self.index, String::from("eof")))
        }
    }
}

pub struct SimpleError {
    _pos: usize,
    _message: String,

}
impl SimpleError {
    pub fn new(pos:usize, message:String)->SimpleError{
        SimpleError{
            _pos: pos,
            _message: message,
        }
    }
}

pub trait Error {
    fn pos(&self)->usize;
    fn message(&self)->&str;
}

impl Error for SimpleError {
    fn pos(&self)->usize {
        self._pos
    }
    fn message(&self)->&str {
        self._message.as_str()
    }
}

impl Debug for SimpleError {
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        let message = format!("<index:{}, mesage:{}>", self.pos(), self.message());
        message.fmt(formatter)
    }
}

pub type Parser<T:'static, R:'static> = Fn(&mut VecState<T>)->Status<R>;
pub type Parsec<T:'static, R:'static> = Arc<Box<Parser<T, R>>>;
pub type Binder<T, C, P> = Arc<Box<Fn(Arc<C>)->Parsec<T, P>>>;
pub type Psc<T:'static> = Arc<Box<T>>;
pub type Status<T:'static> = Result<Arc<T>, SimpleError>;

#[macro_export]
macro_rules! parsec {
    ($x:expr) => (Arc::new(Box::new($x)));
}

pub mod atom;
pub mod combinator;

// pub fn parsec<T, R>(p:&Parser<T, R>)->Parsec<T, R> {
//     Arc::new(Box::new(*p))
// }
