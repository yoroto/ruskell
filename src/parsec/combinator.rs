use parsec::{State, SimpleError};
use std::sync::Arc;

pub fn pack<T, D:'static, S>(data:Arc<D>) -> Box<FnMut(&mut S) -> Result<Arc<D>, SimpleError>> where S:State<T> {
    Box::new(move |_:&mut S|-> Result<Arc<D>, SimpleError> {
        let data=data.clone();
        Ok(data)
    })
}

pub fn try<T, R, S>(mut parsec:Box<FnMut(&mut S) -> Result<Arc<R>, SimpleError>>)
-> Box<FnMut(&mut S) -> Result<Arc<R>, SimpleError>> where S:State<T> {
    Box::new(move |state:&mut S|-> Result<Arc<R>, SimpleError> {
        let pos = state.pos();
        let val = parsec(state);
        if val.is_err() {
            state.seek_to(pos);
        }
        val
    })
}
//
// fn fail<T, S>(msg: String)->Box<FnMut(S) -> Result<(), SimpleError>>> where S:State<T> {
//     Box::new(move |state:S|-> Result<(), SimpleError>> {
//         Err(SimpleError::new(state.pos(), msg))
//     })
// }
//
// struct Either<T, S>{
//     x: Box<FnMut(S) -> Result<(), SimpleError>>>;
//     y: Box<FnMut(S) -> Result<(), SimpleError>>>;
// }



// fn many<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
//
// fn many1<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
