use parsec::{State, SimpleError, Error};
use std::fmt::Display;
use std::sync::Arc;
use std::result;

pub type Result<T> = result::Result<Arc<T>, SimpleError>;
pub type Parsec<T, S> = Box<FnMut(&mut S)->Result<T>>;

pub fn one<T:Eq+Display+'static, S>(x:Arc<T>)->Parsec<T, S> where S:State<T> {
    let value = x.clone();
    Box::new(move |state: &mut S|->Result<T> {
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

pub fn eof<T:Display+'static, S>(state: &mut S)->Result<()> where S:State<T>{
    let val = state.next();
    if val.is_none() {
        Ok(Arc::new(()))
    } else {
        let pos = state.pos();
        let message = format!("expect eof at {} but got {}", pos, val.unwrap());
        Err(SimpleError::new(pos, message))
    }
}
