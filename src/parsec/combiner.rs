use parsec::{State, SimpleError};

fn pack<T, D, S>(data:Arc<D>) -> Box<FnMut(S) -> Result<Arc<D>, SimpleError>>> where S:State<T> {
    Box::new(move |state:S|-> Result<Arc<D>, SimpleError>> {
        let data=data.clone();
        Some(data)
    })
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
