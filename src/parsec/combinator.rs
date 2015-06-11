use parsec::{State, SimpleError};
use std::sync::Arc;
use std::result;

pub type Result<T> = result::Result<Arc<T>, SimpleError>;
pub type Parsec<T, S> = Box<FnMut(&mut S)->Result<T>>;

pub fn pack<T, D:'static, S>(data:Arc<D>) -> Parsec<D, S> where S:State<T> {
    Box::new(move |_:&mut S|-> Result<D> {
        let data=data.clone();
        Ok(data)
    })
}

pub fn try<T, R, S>(mut parsec:Parsec<R, S>) -> Parsec<R, S> where S:State<T> {
    Box::new(move |state:&mut S|-> Result<R> {
        let pos = state.pos();
        let val = parsec(state);
        if val.is_err() {
            state.seek_to(pos);
        }
        val
    })
}

pub fn fail<T, S>(msg: String)->Parsec<(), S> where S:State<T> {
    Box::new(move |state:&mut S|-> Result<()> {
        Err(SimpleError::new(state.pos(), msg.clone()))
    })
}

pub struct Either<R, S>{
    x: Parsec<R, S>,
    y: Parsec<R, S>,
}

fn either<T, R, S>(x: Parsec<R, S>, y:Parsec<R, S>)->Either<R, S> where S:State<T>{
    Either{
        x: x,
        y: y,
    }
}

impl<R, S> FnOnce<(S, )> for Either<R, S> {
    type Output = Parsec<R, S>;
    extern "rust-call" fn call_once(self, args: (&mut S, )) -> Parsec<R, S> {
        let (state, ) = args;
        let pos = state.pos();
        let val = self.x(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                self.y(state)
            } else {
                val
            }
        }
    }
}

impl<R, S> FnMut<(S, )> for Either<R, S> {
    extern "rust-call" fn call_mut(&mut self, args: (&mut S, )) -> Parsec<R, S> {
        self.call_once(args)
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
