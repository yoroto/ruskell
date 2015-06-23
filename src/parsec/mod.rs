use std::vec::Vec;
use std::iter::FromIterator;
use std::sync::Arc;
use std::boxed::Box;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::clone::Clone;

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

//pub trait Parsec<T:'static+Clone, R:'static+Clone>:Debug where Self:Parsec<T, R>+Clone+'static {
pub trait Parsec<T, R>:Debug {
    fn parse(&self, &mut State<T>)->Status<R>;
}
// TODO: move Generic Type Param P to bind/then/over function
// Type Continuation(Result) Then Pass
pub trait M<T:'static, R:'static>:Parsec<T, R> where Self:Clone+'static, T:Clone, R:Clone {
    fn bind<P:'static+Clone>(self, binder:Arc<Box<Fn(&mut State<T>, R)->Status<P>>>)->Monad<T, R, P> {
        Monad::new(Arc::new(self), binder.clone())
    }
    fn then<P:'static+Clone>(self, then:Arc<Parsec<T, P>>)->Monad<T, R, P> {
        let then = then.clone();
        Monad::new(Arc::new(self), Arc::new(Box::new(move |state: &mut State<T>, _:R| {
            let then = then.clone();
            then.parse(state)
        })))
    }
    fn over<P:'static+Clone>(self, over:Arc<Parsec<T, P>>)->Monad<T, R, R> {
        let over = over.clone();
        Monad::new(Arc::new(self), Arc::new(Box::new(move |state: &mut State<T>, x:R| {
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

pub type Status<T> = Result<T, SimpleError>;

// Type Continuation Then Pass
pub struct Monad<T, C, P> {
    parsec: Arc<Parsec<T, C>>,
    binder: Arc<Box<Fn(&mut State<T>, C)->Status<P>>>,
}

impl<T:'static, C:'static, P:'static> Monad<T, C, P>
where T:Clone, P:Clone {
    pub fn new(parsec: Arc<Parsec<T, C>>, binder: Arc<Box<Fn(&mut State<T>, C)->Status<P>>>)-> Monad<T, C, P> {
        Monad{parsec:parsec.clone(), binder:binder.clone()}
    }
}

impl<T, C, P> Parsec<T, P> for Monad<T, C, P>
where T:Clone, P:Clone {
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

impl<'a, T, C, P> FnOnce<(&'a mut State<T>, )> for Monad<T, C, P>
where T:Clone, P:Clone {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<P> {
        panic!("Not implement!");
    }
}

impl<'a, T, C, P> FnMut<(&'a mut State<T>, )> for Monad<T, C, P>
where T:Clone, P:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<P> {
        panic!("Not implement!");
    }
}

impl<'a, T, C, P> Fn<(&'a mut State<T>, )> for Monad<T, C, P>
where T:Clone, P:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<P> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, C, P> Clone for Monad<T, C, P>
where T:Clone, P:Clone {
    fn clone(&self)->Self {
        Monad{parsec:self.parsec.clone(), binder:self.binder.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
        self.binder = source.binder.clone();
    }
}

impl<T, C, P> Debug for Monad<T, C, P> where T:Clone, P:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<monad environment>".fmt(formatter)
    }
}

impl<T:'static, C:'static, P:'static> M<T, P> for Monad<T, C, P> where T:Clone, C:Clone, P:Clone {}

pub fn monad<T:'static, R:'static>(parsec:Arc<Parsec<T, R>>)->Monad<T, R, R> where T:Clone, R:Clone {
    Monad::new(parsec, Arc::new(Box::new(|_:&mut State<T>, re:R| Ok(re))))
}

// A monad just return parsec
pub struct Parser<T, R> {
    parsec: Arc<Parsec<T, R>>,
}

impl<T:'static, R:'static> Parser<T, R>
where T:Clone, R:Clone {
    pub fn new(parsec: Arc<Parsec<T, R>> )-> Parser<T, R> {
        Parser{parsec:parsec.clone()}
    }
}

impl<T, R> Parsec<T, R> for Parser<T, R> where T:Clone, R:Clone {
    fn parse(&self, state: &mut State<T>) -> Status<R> {
        self.parsec.parse(state)
    }
}

impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> Fn<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Parser<T, R> where T:Clone, R:Clone {
    fn clone(&self)->Self {
        Parser{parsec:self.parsec.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
    }
}

impl<T, R> Debug for Parser<T, R> where T:Clone, R:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<parsec monad environment>".fmt(formatter)
    }
}

impl<T:'static, R:'static> M<T, R> for Parser<T, R> where T:Clone, R:Clone {}

pub fn parser<T:'static, R:'static>(parsec:Arc<Parsec<T, R>>)->Parser<T, R> where T:Clone, R:Clone {
    Parser::new(parsec)
}

// A monad just return bind
pub struct Bind<T, R> {
    binder: Arc<Box<Fn(&mut State<T>, T)->Status<R>>>,
}

impl<T:'static, R:'static> Bind<T, R>
where T:Clone, R:Clone {
    pub fn new(binder: Arc<Box<Fn(&mut State<T>, T)->Status<R>>>)-> Bind<T, R> {
        Bind{binder:binder.clone()}
    }
}

impl<T, R> Parsec<T, R> for Bind<T, R> where T:Clone, R:Clone {
    fn parse(&self, state: &mut State<T>) -> Status<R> {
        let n = state.next();
        n.map_or(Err(SimpleError::new(state.pos(), String::from("eof"))),
                |x:T| (self.binder)(state, x))
    }
}

impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Bind<T, R> where T:Clone, R:Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Bind<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> Fn<(&'a mut State<T>, )> for Bind<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Bind<T, R> where T:Clone, R:Clone {
    fn clone(&self)->Self {
        Bind{binder:self.binder.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.binder = source.binder.clone();
    }
}

impl<T, R> Debug for Bind<T, R> where T:Clone, R:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<bind function monad environment>".fmt(formatter)
    }
}

impl<T:'static, R:'static> M<T, R> for Bind<T, R> where T:Clone, R:Clone {}

pub fn bind<T:'static, R:'static>(binder: Arc<Box<Fn(&mut State<T>, T)->Status<R>>>)->Bind<T, R>
where T:Clone, R:Clone {
    Bind::new(binder)
}

#[macro_export]
macro_rules! bnd {
    ($x:expr) => (Arc::new(Box::new($x)));
}

#[macro_export]
macro_rules! arc {
    ($x:expr) => (Arc::new($x));
}

pub mod atom;
pub mod combinator;
pub mod text;
