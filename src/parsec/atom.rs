use parsec::{State, SimpleError};
use std::result::Result;
use std::fmt::Display;
use std::sync::Arc;

pub fn one<T:Eq+Display+'static, S>(x:Arc<T>)->(Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
where S:State<T>{
    let value = x.clone();
    Box::new(move |state: &mut S|->Result<Arc<T>, SimpleError> {
        let value = value.clone();
        let val = state.next_by(&|val:Arc<T>|val==value.clone());
        val.map_or_else(
                ||{
                    let value = value.clone();
                    let pos = state.pos();
                    let message = format!("expect {} at {} but missmatch", value, pos);
                    Err(SimpleError::new(pos, message))
                },
                |x:Arc<T>|->Result<Arc<T>, SimpleError>{Ok(x)},
            )
    })
}

pub fn eof<T:Display+'static, S>(state: &mut S)->Result<(), SimpleError> where S:State<T>{
    let val = state.next();
    if val.is_none() {
        Ok(())
    } else {
        let pos = state.pos();
        let message = format!("expect eof at {} but got {}", pos, val.unwrap());
        Err(SimpleError::new(pos, message))
    }
}
