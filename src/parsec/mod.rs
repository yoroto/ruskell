use std::vec::Vec;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::sync::Arc;
use std::fmt::{Debug, Formatter};
use std::fmt;

pub struct VecState<T> {
    index : usize,
    buffer: Vec<T>,
}

impl<A> FromIterator<A> for VecState<A> {
    fn from_iter<T>(iterator: T) -> Self where T:IntoIterator<Item=A> {
        VecState{
            index:0,
            buffer:Vec::from_iter(iterator.into_iter()),
        }
    }
}

pub trait State<T> {
    fn pos(&self)-> usize;
    fn seek_to(&mut self, usize)->bool;
    fn next(&mut self)->Option<T>;
    fn next_by(&mut self, &Fn(&T)->bool)->Status<T>;
}

impl<T> State<T> for VecState<T> where T:Clone {
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
    fn next(&mut self)->Option<T>{
        if 0 as usize <= self.index && self.index < self.buffer.len() {
            let item = self.buffer[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    fn next_by(&mut self, pred:&Fn(&T)->bool)->Status<T>{
        if 0 as usize <= self.index && self.index < self.buffer.len() {
            let ref item = self.buffer[self.index];
            if pred(item) {
                self.index += 1;
                Ok(item.clone())
            } else {
                Err(SimpleError::new(self.index, String::from("predicate failed")))
            }
        } else {
            Err(SimpleError::new(self.index, String::from("eof")))
        }
    }
}

#[derive(Debug, Clone)]
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

pub trait Parsec<T, R> {
    fn parse(&self, &mut State<T>)->Status<R>;
}

pub type Status<T> = Result<T, SimpleError>;

// Type Continuation Then Pass
pub struct Monad<T, C, P, PTC> {
    parsec: PTC, //Parsec<T, C>,
    binder: Arc<Box<Fn(&mut State<T>, C)->Status<P>>>,
    ttype: PhantomData<T>,
    ctype: PhantomData<C>,
    ptype: PhantomData<P>,
}

impl<T, C, P, PTC> Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    pub fn new(parsec: PTC, binder: Arc<Box<Fn(&mut State<T>, C)->Status<P>>>)-> Monad<T, C, P, PTC> {
        Monad{parsec:parsec.clone(), binder:binder.clone(),
            ttype:PhantomData, ctype:PhantomData, ptype:PhantomData}
    }
    pub fn bind<R>(self, binder:Arc<Box<Fn(&mut State<T>, P)->Status<R>>>)->Monad<T, P, R, Self>
    where R:Clone {
        Monad::new(self, binder.clone())
    }

    pub fn then<R, Then:'static>(&self, then:Then)->Monad<T, P, R, Self>
    where R:Clone, Then:Parsec<T, R>+Clone {
        let then = then.clone();
        let s = self.clone();
        Monad::new(s, Arc::new(Box::new(move |state: &mut State<T>, _:P| {
            let then = then.clone();
            then.parse(state)
        })))
    }

    pub fn over<R, Over:'static>(&self, over:Over)->Monad<T, P, P, Self>
    where R:Clone, Over:Parsec<T, R>+Clone {
        let over = over.clone();
        let s = self.clone();
        Monad::new(s, Arc::new(Box::new(move |state: &mut State<T>, x:P| {
            let over = over.clone();
            let re = over.parse(state);
            if re.is_ok() {
                Ok(x)
            } else {
                Err(re.err().unwrap())
            }
        })))
    }
}

impl<T, C, P, PTC> Parsec<T, P> for Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    fn parse(&self, state: &mut State<T>) -> Status<P> {
        let x = self.parsec.parse(state);
        if x.is_ok() {
            let pre = x.unwrap();
            let re = (self.binder.clone())(state, pre);
            re
        } else {
            Err(x.err().unwrap())
        }
    }
}

impl<'a, T, C, P, PTC> FnOnce<(&'a mut VecState<T>, )> for Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<P> {
        panic!("Not implement!");
    }
}

impl<'a, T, C, P, PTC> FnMut<(&'a mut VecState<T>, )> for Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut VecState<T>, )) -> Status<P> {
        panic!("Not implement!");
    }
}

impl<'a, T, C, P, PTC> Fn<(&'a mut VecState<T>, )> for Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, C, P, PTC> Clone for Monad<T, C, P, PTC>
where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone {
    fn clone(&self)->Self {
        Monad{parsec:self.parsec.clone(), binder:self.binder.clone(),
            ttype:PhantomData, ctype:PhantomData, ptype:PhantomData}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
        self.binder = source.binder.clone();
    }
}

impl<T, C, P, PTC> Debug for Monad<T, C, P, PTC> where T:Clone, P:Clone, PTC:Parsec<T, C>+Clone+Debug{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<monad environment>".fmt(formatter)
    }
}

pub fn monad<T, R, P>(parsec:P)->Monad<T, R, R, P> where P:Parsec<T, R>+Clone, T:Clone, R:Clone {
    Monad::new(parsec, Arc::new(Box::new(|_:&mut State<T>, re:R| Ok(re))))
}

pub mod atom;
pub mod combinator;
