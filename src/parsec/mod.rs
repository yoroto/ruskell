use std::vec::Vec;
use std::iter::FromIterator;
use std::sync::Arc;
use std::boxed::Box;
// use std::fmt::{Debug, Formatter, Display};
use std::fmt::{Formatter, Display};
use std::fmt;
use std::clone::Clone;
use std::convert::{From};
use std::error;

pub trait State<T> {
    fn pos(&self)-> usize;
    fn seek_to(&mut self, usize)->bool;
    fn next(&mut self)->Option<T>;
    fn next_by(&mut self, &Fn(&T)->bool)->Status<T>;
    fn err(&self, description:String)->ParsecError {
        ParsecError::new(self.pos(), description)
    }
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
            self.index += 1;
            if pred(item) {
                Ok(item.clone())
            } else {
                Err(self.err(String::from("predicate failed")))
            }
        } else {
            Err(self.err(String::from("eof")))
        }
    }
}

pub trait Error:error::Error {
    fn pos(&self)->usize;
}

#[derive(Debug, Clone)]
pub struct ParsecError {
    _pos: usize,
    message: String,

}

impl ParsecError {
    pub fn new(pos:usize, description:String)->ParsecError{
        ParsecError{
            _pos: pos,
            message: description,
        }
    }
}

impl Error for ParsecError {
    fn pos(&self)->usize {
        self._pos
    }
}
impl error::Error for ParsecError {
    fn description(&self)->&str {
        self.message.as_str()
    }
    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

impl Display for ParsecError {
    fn fmt(&self, formatter:&mut Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.message)
    }
}

//pub trait Parsec<T:'static+Clone, R:'static+Clone>:Debug where Self:Parsec<T, R>+Clone+'static {
pub trait Parsec<T, R> {
    fn parse(&self, &mut State<T>)->Status<R>;
}

// Type Continuation(Result) Then Pass
pub trait Monad<T:'static, R:'static>:Parsec<T, R> where Self:Clone+'static, T:Clone, R:Clone {
    fn bind<P:'static+Clone>(self, binder:Arc<Box<Fn(&mut State<T>, R)->Status<P>>>)->Parser<T, P> {
        abc!(move |state:&mut State<T>|->Status<P>{
            let pre = self.parse(state);
            if pre.is_err() {
                return Err(pre.err().unwrap())
            }
            let binder = binder.clone();
            binder(state, pre.ok().unwrap())
        })
    }
    fn then<P:'static+Clone, Thn:'static>(self, then:Thn)->Parser<T, P>
    where Thn:Parsec<T, P>+Clone{
        let then = then.clone();
        abc!(move |state:&mut State<T>|->Status<P>{
            let pre = self.parse(state);
            if pre.is_err() {
                return Err(pre.err().unwrap())
            }
            then.parse(state)
        })
    }
    fn over<P:'static+Clone, Ovr:'static>(self, over:Ovr)->Parser<T, R>
    where Ovr:Parsec<T, P>+Clone{
        let over = over.clone();
        abc!(move |state:&mut State<T>|->Status<R>{
            let re = self.parse(state);
            if re.is_err() {
                return re;
            }
            let o = over.parse(state);
            if o.is_err() {
                return Err(o.err().unwrap())
            }
            Ok(re.ok().unwrap())
        })
    }
}

pub type Status<T> = Result<T, ParsecError>;

// A monad just return closure
// pub struct Parser<T, R> {
//     parser: Arc<Box<Fn(&mut State<T>)->Status<R>>>,
// }

pub type Parser<T, R> = Arc<Box<Fn(&mut State<T>)->Status<R>>>;

// impl<T:'static, R:'static> Parser<T, R>
// where T:Clone, R:Clone {
//     pub fn new(parser: Arc<Box<Fn(&mut State<T>)->Status<R>>>)-> Parser<T, R> {
//         Parser{parser:parser.clone()}
//     }
// }

impl<T, R> Parsec<T, R> for Parser<T, R> where T:Clone, R:Clone {
    fn parse(&self, state: &mut State<T>) -> Status<R> {
        self(state)
    }
}

// impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
//     type Output = Status<R>;
//     extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
//         panic!("Not implement!");
//     }
// }
//
// impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
//     extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
//         panic!("Not implement!");
//     }
// }
//
// impl<'a, T, R> Fn<(&'a mut State<T>, )> for Parser<T, R> where T:Clone, R:Clone {
//     extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
//         let (state, ) = args;
//         self.parse(state)
//     }
// }

// impl<T, R> Clone for Parser<T, R> where T:Clone, R:Clone {
//     fn clone(&self)->Self {
//         Parser{parser:self.parser.clone()}
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         self.parser = source.parser.clone();
//     }
// }

// impl<T, R> Debug for Parser<T, R> where T:Clone, R:Clone{
//     fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
//         write!(formatter, "<closure parser monad environment>")
//     }
// }

impl<T:'static, R:'static> Monad<T, R> for Parser<T, R> where T:Clone, R:Clone {}

// pub fn parser<T:'static, R:'static>(parser: Arc<Box<Fn(&mut State<T>)->Status<R>>>)->Parser<T, R>
// where T:Clone, R:Clone {
//     Parser::new(parser)
// }


pub mod atom;
pub mod combinator;
// pub mod text;
