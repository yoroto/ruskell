use parsec::{VecState, State, SimpleError, Error, Parsec, Status};
use std::fmt::Display;
use std::sync::Arc;

pub fn one<T:Eq+Display+'static>(x:Arc<T>)->Parsec<T, T>{
    let value = x.clone();
    Box::new(move |state: &mut VecState<T>|->Status<T> {
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
    Box::new(move |state: &mut VecState<T>|->Status<()> {
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
