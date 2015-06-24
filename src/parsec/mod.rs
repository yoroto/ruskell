use std::vec::Vec;
use std::iter::FromIterator;
use std::sync::Arc;
use std::boxed::Box;
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::clone::Clone;

//Arc<Box<Closure>>
#[macro_export]
macro_rules! abc {
    ($x:expr) => (Arc::new(Box::new($x)));
}

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
pub trait Monad<T:'static, R:'static>:Parsec<T, R> where Self:Clone+'static, T:Clone, R:Clone {
    fn bind<P:'static+Clone>(self, binder:Arc<Box<Fn(&mut State<T>, R)->Status<P>>>)->Parser<T, P> {
        parser(abc!(move |state:&mut State<T>|->Status<P>{
            let pre = self.parse(state);
            if pre.is_err() {
                return Err(pre.err().unwrap())
            }
            let binder = binder.clone();
            binder(state, pre.ok().unwrap())
        }))
    }
    fn then<P:'static+Clone, Thn:'static>(self, then:Thn)->Parser<T, P>
    where Thn:Parsec<T, P>+Clone{
        let then = then.clone();
        parser(abc!(move |state:&mut State<T>|->Status<P>{
            let pre = self.parse(state);
            if pre.is_err() {
                return Err(pre.err().unwrap())
            }
            then.parse(state)
        }))
    }
    fn over<P:'static+Clone, Ovr:'static>(self, over:Ovr)->Parser<T, R>
    where Ovr:Parsec<T, P>+Clone{
        let over = over.clone();
        parser(abc!(move |state:&mut State<T>|->Status<R>{
            let re = self.parse(state);
            if re.is_err() {
                return re;
            }
            let o = over.parse(state);
            if o.is_err() {
                return Err(o.err().unwrap())
            }
            Ok(re.ok().unwrap())
        }))
    }
}

pub type Status<T> = Result<T, SimpleError>;

// A monad just return closure
pub struct Parser<T, R> {
    parserer: Arc<Box<Fn(&mut State<T>)->Status<R>>>,
}

impl<T:'static, R:'static> Parser<T, R>
where T:Clone, R:Clone {
    pub fn new(parserer: Arc<Box<Fn(&mut State<T>)->Status<R>>>)-> Parser<T, R> {
        Parser{parserer:parserer.clone()}
    }
}

impl<T, R> Parsec<T, R> for Parser<T, R> where T:Clone, R:Clone {
    fn parse(&self, state: &mut State<T>) -> Status<R> {
        (self.parserer)(state)
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
        Parser{parserer:self.parserer.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parserer = source.parserer.clone();
    }
}

impl<T, R> Debug for Parser<T, R> where T:Clone, R:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<closure parserer monad environment>".fmt(formatter)
    }
}

impl<T:'static, R:'static> Monad<T, R> for Parser<T, R> where T:Clone, R:Clone {}

pub fn parser<T:'static, R:'static>(parserer: Arc<Box<Fn(&mut State<T>)->Status<R>>>)->Parser<T, R>
where T:Clone, R:Clone {
    Parser::new(parserer)
}


pub mod atom;
pub mod combinator;
pub mod text;
