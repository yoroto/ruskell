use parsec::{State, SimpleError};

fn pack<T, D, S>(data:Arc<D>) -> Box<FnMut(S) -> Result<Arc<D>, SimpleError>>> where S:State<T> {
    Box::new(move |state:S|-> Result<Arc<D>, SimpleError>> {
        let data=data.clone();
        Some(data)
    })
}

fn try<T, P, S>(parsec:FnMut(S) -> Result<Arc<P>, SimpleError>>)
-> Box<FnMut(S) -> Result<Arc<P>, SimpleError>>> where S:State<T> {
    Box::new(move |state:S|-> Result<Arc<P>, SimpleError>> {
        let pos = state.pos();
        let val = parsec(state);
        if val.is_err() {
            state.seek_to(pos);
        }
        val
    })
}

fn fail<T, S>(msg: String)->Box<FnMut(S) -> Result<(), SimpleError>>> where S:State<T> {
    Box::new(move |state:S|-> Result<(), SimpleError>> {
        Err(SimpleError::new(state.pos(), msg))
    })
}

struct Either<T, S>{
    x: Box<FnMut(S) -> Result<(), SimpleError>>>;
    y: Box<FnMut(S) -> Result<(), SimpleError>>>;
}

impl

// fn many<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
//
// fn many1<T, S>(parsec: Box<FnMut(&mut S)->Result<Arc<T>, SimpleError>>)
//     -> Box<FnMut(&mut S)->Result<Vec<Arc<T>>, SimpleError>> {
//
// }
