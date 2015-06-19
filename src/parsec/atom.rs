use parsec::{State, SimpleError, Error, Parsec, Status};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct One<T>{
    element : T,
}

impl<T> One<T> where T:Eq+Display+Clone {
    fn new(element:T) -> One<T> {
        One{element:element}
    }
}

impl<T> Parsec<T, T> for One<T> where T:Eq+Display+Clone {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        let ref value = self.element;
        let val = state.next_by(&|val:&T|val.eq(value));
        val.map_err(
                |err:SimpleError|{
                    let pos = state.pos();
                    let message = format!("expect {} at {} but missmatch: {}", self.element.clone(),
                        pos, err.message());
                    SimpleError::new(pos, message)
                })
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for One<T> {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for One<T> {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for One<T> where T:Eq+Display+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn one<T>(element:T) -> One<T> where T:Eq+Display+Clone {
    One::new(element)
}

#[derive(Debug, Clone)]
pub struct Eof<T>{
    data: PhantomData<T>,
}

impl<T> Eof<T>{
    fn new() -> Eof<T> {
        Eof{data:PhantomData}
    }
}

impl<T> Parsec<T, ()> for Eof<T> where T:Clone+Display {
    fn parse(&self, state:&mut State<T>)->Status<()>{
        let val = state.next();
        if val.is_none() {
            Ok(())
        } else {
            let pos = state.pos();
            let message = format!("expect eof at {} but got value {}", pos, val.unwrap());
            Err(SimpleError::new(pos, message))
        }
    }
}

impl<'a, S, T> FnOnce<(&'a mut S, )> for Eof<T> where S:State<T>{
    type Output = Status<()>;
    extern "rust-call" fn call_once(self, _: (&'a mut S, )) -> Status<()> {
        panic!("Not implement!");
    }
}

impl<'a, S, T> FnMut<(&'a mut S, )> for Eof<T> where S:State<T>{
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut S, )) -> Status<()> {
        panic!("Not implement!");
    }
}

impl<'a, S, T> Fn<(&'a mut S, )> for Eof<T> where T:Clone+Display, S:State<T> {
    extern "rust-call" fn call(&self, args: (&'a mut S, )) -> Status<()> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn eof<T>() -> Eof<T> {
    Eof::new()
}

#[derive(Debug, Clone)]
pub struct OneOf<T> {
    elements: Vec<T>,
}

impl<T> OneOf<T> where T:Eq+Display+Clone+Debug {
    pub fn new(elements:&Vec<T>) -> OneOf<T> {
        let mut es = Vec::new();
        es.push_all(&elements);
        OneOf{elements:es}
    }
}

impl<T> Parsec<T, T> for OneOf<T> where T:Eq+Display+Clone+Debug {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        let next = state.next();
        if next.is_none() {
            Err(SimpleError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in self.elements.iter() {
                if d == &it {
                    return Ok(it);
                }
            }
            let message = format!("<expect one of {:?}, got:{}>", self.elements, it);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for OneOf<T> {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for OneOf<T> {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for OneOf<T> where T:Eq+Clone+Display+Debug {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn one_of<T:'static+Eq+Debug+Display>(elements:&Vec<T>)->OneOf<T>
        where T:Eq+Display+Clone+Debug {
    OneOf::new(&elements)
}

#[derive(Debug, Clone)]
pub struct NoneOf<T> {
    elements: Vec<T>,
}

impl<T> NoneOf<T> where T:Eq+Display+Clone+Debug {
    pub fn new(elements:&Vec<T>) -> NoneOf<T> {
        let mut es = Vec::new();
        es.push_all(&elements);
        NoneOf{elements:es}
    }
}

impl<T> Parsec<T, T> for NoneOf<T> where T:Eq+Display+Clone+Debug {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        let next = state.next();
        if next.is_none() {
            Err(SimpleError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in self.elements.iter() {
                if d == &it {
                    let message = format!("<expect none of {:?}, got:{}>", self.elements, it);
                    return Err(SimpleError::new(state.pos(), String::from(message)))
                }
            }
            return Ok(it);
        }
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for NoneOf<T> {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for NoneOf<T> {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for NoneOf<T> where T:Eq+Clone+Display+Debug {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn none_of<T:'static+Eq+Debug+Display>(elements:&Vec<T>)->NoneOf<T>
        where T:Eq+Display+Clone+Debug {
    NoneOf::new(&elements)
}

#[derive(Debug, Clone)]
pub struct Pack<I, T>{
    element : T,
    input_type: PhantomData<I>,
}

impl<I, T> Pack<I, T> where I: Clone, T:Clone {
    fn new(element:T) -> Pack<I, T> {
        Pack{element:element, input_type:PhantomData}
    }
}

impl<I, T> Parsec<I, T> for Pack<I, T> where I: Clone, T:Clone {
    fn parse(&self, _:&mut State<I>)->Status<T> {
        Ok(self.element.clone())
    }
}

impl<'a, I, T> FnOnce<(&'a mut State<I>, )> for Pack<I, T> where I: Clone, T:Clone {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<I>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, I, T> FnMut<(&'a mut State<I>, )> for Pack<I, T> where I: Clone, T:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<I>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, I, T> Fn<(&'a mut State<I>, )> for Pack<I, T> where I: Clone, T:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<I>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn pack<I, T>(element:T) -> Pack<I, T> where I: Clone, T:Clone {
    Pack::new(element)
}

#[derive(Debug, Clone)]
pub struct Fail<I>{
    message:Arc<String>,
    input_type: PhantomData<I>,
}

impl<I> Fail<I> where I: Clone {
    fn new(message:String) -> Fail<I> {
        let msg = Arc::new(message);
        Fail{message:msg, input_type:PhantomData}
    }
}

impl<I> Parsec<I, ()> for Fail<I> where I: Clone {
    fn parse(&self, state:&mut State<I>)->Status<()>{
        Err(SimpleError::new(state.pos(), String::from(self.message.as_str())))
    }
}

impl<'a, I> FnOnce<(&'a mut State<I>, )> for Fail<I> where I: Clone {
    type Output = Status<()>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<I>, )) -> Status<()> {
        panic!("Not implement!");
    }
}

impl<'a, I> FnMut<(&'a mut State<I>, )> for Fail<I> where I: Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<I>, )) -> Status<()> {
        panic!("Not implement!");
    }
}

impl<'a, I> Fn<(&'a mut State<I>, )> for Fail<I> where I: Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<I>, )) -> Status<()> {
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn fail<I>(message:String) -> Fail<I> where I: Clone {
    Fail::new(message)
}
