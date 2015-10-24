use parsec::{State, ParsecError, Error, Status, Parser};
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub fn one<T:'static>()->Parser<T, T> {
    abc!(|state:&mut State<T>|->Status<T>{
        state.next().ok_or(ParsecError::new(state.pos(), String::from("eof")))
    })
}

pub fn eq<T:'static>(val:T) -> Parser<T, T> where T:Eq+Display+Debug+Clone {
    abc!(move |state:&mut State<T>|->Status<T>{
        let value = state.next();
        let pos = state.pos();
        if value.is_some() {
            let x = value.unwrap();
            if x == val {
                return Ok(x);
            }
            let val = val.clone();
            let description = format!("expect {} equal element {} at {}", val, x, pos);
            return Err(ParsecError::new(pos, description));
        }
        Err(ParsecError::new(pos, String::from("eof")))
    })
}


pub fn ne<T:'static>(val:T) -> Parser<T, T> where T:Display+Eq+Debug+Clone {
    abc!(move |state:&mut State<T>|->Status<T>{
        let value = state.next();
        let pos = state.pos();
        if value.is_some() {
            let x = value.unwrap();
            if x == val {
                let val = val.clone();
                let description = format!("expect {} not equal element {} at {}", val, x, pos);
                return Err(ParsecError::new(pos, description));
            }
            return Ok(x);
        }
        Err(ParsecError::new(pos, String::from("eof")))
    })
}

pub fn eof<T:'static+Display>()->Parser<T, ()> {
    abc!(|state: &mut State<T>|->Status<()> {
        let val = state.next();
        if val.is_none() {
            Ok(())
        } else {
            let pos = state.pos();
            let description = format!("expect eof at {} but got value {}", pos, val.unwrap());
            Err(ParsecError::new(pos, description))
        }
    })
}

pub fn one_of<T:Eq+Debug+Display+Clone+'static>(elements:&Vec<T>) -> Parser<T, T> {
    let elements = elements.clone();
    abc!(move |state: &mut State<T>|->Status<T>{
        let next = state.next();
        if next.is_none() {
            Err(ParsecError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in elements.iter() {
                if d == &it {
                    return Ok(it);
                }
            }
            let description = format!("<expect one of {:?} at {}, got:{}>", elements, state.pos(), it);
            Err(ParsecError::new(state.pos(), String::from(description)))
        }
    })
}

pub fn none_of<T:Eq+Debug+Display+Clone+'static>(elements:&Vec<T>) -> Parser<T, T> {
    let elements = elements.clone();
    abc!(move |state: &mut State<T>|->Status<T> {
        let next = state.next();
        if next.is_none() {
            Err(ParsecError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in elements.iter() {
                if d == &it {
                    let description = format!("<expect none of {:?} at {}, got:{}>", elements, state.pos(), it);
                    return Err(ParsecError::new(state.pos(), String::from(description)))
                }
            }
            return Ok(it);
        }
    })
}

pub fn pack<T, R:Clone+'static>(element:R) -> Parser<T, R> {
    abc!(move |_: &mut State<T>|->Status<R>{
        Ok(element.clone())
    })
}

pub fn fail<T:'static+Clone, R>(description:String) -> Parser<T, R> {
    abc!(move |state:&mut State<T>|->Status<R>{
        Err(ParsecError::new(state.pos(), String::from(description.as_str())))
    })
}
