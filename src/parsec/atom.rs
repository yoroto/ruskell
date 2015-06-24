use parsec::{State, SimpleError, Error, Parsec, Status, Monad};
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::sync::Arc;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct One<T>{
    input : PhantomData<T>,
}

impl<T> One <T> where T:Debug+Clone {
    fn new() -> One<T> {
        One{input:PhantomData}
    }
}

impl<T> Parsec<T, T> for One<T> where T:Debug+Clone {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        state.next().ok_or(SimpleError::new(state.pos(), String::from("eof")))
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for One<T> where T:Debug+Clone {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for One<T> where T:Debug+Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for One<T> where T:Debug+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T:'static+Debug+Clone> Monad<T, T> for One<T>{}

pub fn one<T>() -> One<T> where T:Debug+Clone {
    One::new()
}

#[derive(Debug, Clone)]
pub struct Equal<T>{
    element : T,
}

impl<T> Equal<T> where T:Eq+Display+Debug+Clone {
    fn new(element:T) -> Equal<T> {
        Equal{element:element}
    }
}

impl<T> Parsec<T, T> for Equal<T> where T:Eq+Display+Debug+Clone {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        let ref value = self.element;
        let val = state.next_by(&|val:&T|val.eq(value));
        val.map_err(
                |_:SimpleError|{
                    let pos = state.pos();
                    let element = self.element.clone();
                    let message = format!("expect {} at {} but missmatch", element, pos);
                    SimpleError::new(pos, message)
                })
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for Equal<T> where T:Eq+Display+Debug+Clone {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for Equal<T> where T:Eq+Display+Debug+Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for Equal<T> where T:Eq+Display+Debug+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T:'static+Eq+Display+Debug+Clone> Monad<T, T> for Equal<T>{}

pub fn eq<T>(element:T) -> Equal<T> where T:Eq+Display+Debug+Clone {
    Equal::new(element)
}

#[derive(Debug, Clone)]
pub struct NotEqual<T>{
    element : T,
}

impl<T> NotEqual<T> where T:Eq+Display+Debug+Clone {
    fn new(element:T) -> NotEqual<T> {
        NotEqual{element:element}
    }
}

impl<T> Parsec<T, T> for NotEqual<T> where T:Eq+Display+Debug+Clone {
    fn parse(&self, state:&mut State<T>)->Status<T>{
        let ref value = self.element;
        let val = state.next_by(&|val:&T|val.ne(value));
        val.map_err(
                |_:SimpleError|{
                    let pos = state.pos();
                    let element = self.element.clone();
                    let message = format!("expect {} not equal element at {}", element, pos);
                    SimpleError::new(pos, message)
                })
    }
}

impl<'a, T> FnOnce<(&'a mut State<T>, )> for NotEqual<T> where T:Eq+Display+Debug+Clone {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> FnMut<(&'a mut State<T>, )> for NotEqual<T> where T:Eq+Display+Debug+Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, T> Fn<(&'a mut State<T>, )> for NotEqual<T> where T:Eq+Display+Debug+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T:'static+Eq+Display+Debug+Clone> Monad<T, T> for NotEqual<T>{}

pub fn ne<T>(element:T) -> NotEqual<T> where T:Eq+Display+Debug+Clone {
    NotEqual::new(element)
}

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

impl<T> Clone for Eof<T> where T:Clone {
    fn clone(&self)->Self {
        eof::<T>()
    }

    fn clone_from(&mut self, _: &Self) {
    }
}

impl<T> Debug for Eof<T> where T:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        write!(formatter, "<eof parsec>")
    }
}

impl<T:'static+Debug+Display+Clone> Monad<T, ()> for Eof<T>{}

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

impl<T:'static+Eq+Debug+Display+Clone> Monad<T, T> for OneOf<T>{}

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

impl<T:'static+Eq+Debug+Display+Clone> Monad<T, T> for NoneOf<T>{}

pub fn none_of<T:'static+Eq+Debug+Display>(elements:&Vec<T>)->NoneOf<T>
        where T:Eq+Display+Clone+Debug {
    NoneOf::new(&elements)
}

pub struct Pack<I, T>{
    element : T,
    input_type: PhantomData<I>,
}

impl<I, T> Pack<I, T> where T:Clone+Debug {
    fn new(element:T) -> Pack<I, T> {
        Pack{element:element, input_type:PhantomData}
    }
}

impl<I, T> Parsec<I, T> for Pack<I, T> where T:Clone+Debug {
    fn parse(&self, _:&mut State<I>)->Status<T> {
        Ok(self.element.clone())
    }
}

impl<'a, I, T> FnOnce<(&'a mut State<I>, )> for Pack<I, T> where T:Clone+Debug {
    type Output = Status<T>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<I>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, I, T> FnMut<(&'a mut State<I>, )> for Pack<I, T> where T:Clone+Debug {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<I>, )) -> Status<T> {
        panic!("Not implement!");
    }
}

impl<'a, I, T> Fn<(&'a mut State<I>, )> for Pack<I, T> where T:Clone+Debug {
    extern "rust-call" fn call(&self, args: (&'a mut State<I>, )) -> Status<T> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<I, T> Clone for Pack<I, T> where T:Clone+Debug {
    fn clone(&self)->Self {
        Pack{element:self.element.clone(), input_type:PhantomData}
    }

    fn clone_from(&mut self, source: &Self) {
        self.element = source.element.clone();
    }
}

impl<I, T> Debug for Pack<I, T> where T:Clone+Debug {
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        write!(formatter, "<pack parsec({:?})>", self.element)
    }
}

impl<I:'static+Clone, T:'static+Debug+Clone> Monad<I, T> for Pack<I, T>{}

pub fn pack<I, T>(element:T) -> Pack<I, T> where T:Clone+Debug {
    Pack::new(element)
}

pub struct Fail<T, R>{
    message:Arc<String>,
    input_type: PhantomData<T>,
    output_type: PhantomData<R>,
}

impl<T, R> Fail<T, R> where T: Clone, R:Clone {
    fn new(message:String) -> Fail<T, R> {
        let msg = Arc::new(message);
        Fail{message:msg, input_type:PhantomData, output_type:PhantomData}
    }
}

impl<T, R> Parsec<T, R> for Fail<T, R> where T:Clone, R: Clone {
    fn parse(&self, state:&mut State<T>)->Status<R>{
        Err(SimpleError::new(state.pos(), String::from(self.message.as_str())))
    }
}

impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Fail<T, R> where T:Clone, R:Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Fail<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> Fn<(&'a mut State<T>, )> for Fail<T, R> where T:Clone, R:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Fail<T, R>{
    fn clone(&self)->Self {
        Fail{message:self.message.clone(), input_type:PhantomData, output_type:PhantomData}
    }

    fn clone_from(&mut self, source: &Self) {
        self.message = source.message.clone();
    }
}

impl<T, R> Debug for Fail<T, R> where T:Clone, R:Clone {
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        write!(formatter, "<fail parsec: {:?}>", self.message)
    }
}

impl<T:'static, R:'static> Monad<T, R> for Fail<T, R> where T:Clone, R:Clone{}

pub fn fail<T, R>(message:String) -> Fail<T, R> where T:Clone, R:Clone {
    Fail::new(message)
}
