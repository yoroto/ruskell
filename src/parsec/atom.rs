use parsec::{VecState, State, SimpleError, Error, Parsec, Status};
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::ops::Deref;

pub fn one<T:Eq+Display+'static>(x:Arc<T>)->Parsec<T, T>{
    let value = x.clone();
    parsec!(move |state: &mut VecState<T>|->Status<T> {
        let value = value.clone();
        let val = state.next_by(&|val:Arc<T>|val==value.clone());
        val.map_err(
                |err:SimpleError|{
                    let value = value.clone();
                    let pos = state.pos();
                    let message = format!("expect {} at {} but missmatch: {}", value, pos, err.message());
                    SimpleError::new(pos, message)
                }
            )
    })
}

pub fn eof<T>()->Parsec<T, ()> {
    parsec!(move |state: &mut VecState<T>|->Status<()> {
        let mut state = state;
        let val = state.next();
        if val.is_none() {
            Ok(Arc::new(()))
        } else {
            let pos = state.pos();
            let message = format!("expect eof at {} but got somthing", pos);
            Err(SimpleError::new(pos, message))
        }
    })
}

pub fn one_of<T:'static+Eq+Debug+Display>(data:Arc<Vec<T>>)->Parsec<T, T> {
    let data = data.clone();
    parsec!(move |state: &mut VecState<T>|->Status<T>{
        let data = data.clone();
        let next = state.next();
        if next.is_none() {
            Err(SimpleError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in data.iter() {
                if d == it.deref() {
                    return Ok(it);
                }
            }
            let message = format!("<expect one of {:?}, got:{}>", data, it);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    })
}

pub fn none_of<T:'static+Eq+Debug+Display>(data:Arc<Vec<T>>)->Parsec<T, T> {
    let data = data.clone();
    parsec!(move |state: &mut VecState<T>|->Status<T>{
        let data = data.clone();
        let next = state.next();
        if next.is_none() {
            Err(SimpleError::new(state.pos(), String::from("eof")))
        } else {
            let it = next.unwrap();
            for d in data.iter() {
                if d == it.deref() {
                    let message = format!("<expect none of {:?}, got:{}>", data, it);
                    return Err(SimpleError::new(state.pos(), String::from(message)));
                }
            }
            return Ok(it);
        }
    })
}
