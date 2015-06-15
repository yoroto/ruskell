use parsec::{VecState, State, SimpleError};
use std::sync::Arc;

pub type Status<T> = Result<Arc<T>, SimpleError>;
pub type Parsec<T, R> = Box<FnMut(&mut VecState<T>)->Status<R>>;

pub fn pack<T, R:'static>(data:Arc<R>) -> Parsec<T, R> {
    Box::new(move |_:&mut VecState<T>|-> Status<R> {
        let data=data.clone();
        Ok(data)
    })
}

pub fn try<T, R>(mut parsec:Parsec<T, R>) -> Parsec<T, R> {
    Box::new(move |state:&mut VecState<T>|-> Status<R> {
        let pos = state.pos();
        let val = parsec(state);
        if val.is_err() {
            state.seek_to(pos);
        }
        val
    })
}

pub fn fail<T>(msg: String)->Parsec<T, ()> {
    Box::new(move |state:&mut VecState<T>|-> Status<()> {
        Err(SimpleError::new(state.pos(), msg.clone()))
    })
}

pub struct Either<T, R>{
    x: Parsec<T, R>,
    y: Parsec<T, R>,
}

pub fn either<T, R>(x: Parsec<T, R>, y: Parsec<T, R>)->Either<T, R> {
    Either{
        x: x,
        y: y,
    }
}

impl<'a, T, R> FnOnce<(&'a mut VecState<T>, )> for Either<T, R> {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<R> {
        let (state, ) = args;
        let pos = state.pos();
        let mut fx = self.x;
        let val = (fx)(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                let mut fy = self.y;
                (fy)(state)
            } else {
                val
            }
        }
    }
}

impl<'a, T, R> FnMut<(&'a mut VecState<T>, )> for Either<T, R> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<R> {
        //self.call_once(args)
        let (state, ) = args;
        let pos = state.pos();
        let val = (self.x)(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                (self.y)(state)
            } else {
                val
            }
        }
    }
}

impl<T:'static, R:'static> Either<T, R> {
    pub fn or(self, p:Parsec<T, R>) -> Self {
        let re = either(Box::new(self), p);
        re
    }
}


// fn many<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
//
// fn many1<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
