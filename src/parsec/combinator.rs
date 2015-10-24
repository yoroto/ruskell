use parsec::{State, Parsec, Status, Monad, Parser};
use parsec::atom::{pack, fail};
use std::fmt::{Debug};
use std::sync::Arc;

pub fn try<T:'static, R:'static, X:'static>(p:X)->Parser<T, R>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    abc!(move |state: &mut State<T>|->Status<R>{
        let pos = state.pos();
        let res = p.parse(state);
        if res.is_err() {
            state.seek_to(pos);
        }
        res
    })
}

pub trait Or<T, R> {
    fn or(&self, Parser<T, R>)->Parser<T, R>;
}

pub type Either<T, R> = Arc<Box<Fn(&mut State<T>)->Status<R>>>;
pub fn either<T, R, X:'static, Y:'static>(x:X, y:Y)->Either<T, R>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone{
    let x = x.clone();
    let y = y.clone();
    abc!(move |state:&mut State<T>|->Status<R>{
        let pos = state.pos();
        let val = x.parse(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                y.parse(state)
            } else {
                val
            }
        }
    })
}
impl<T:'static+Clone, R:'static+Clone> Or<T, R> for Either<T, R> {
    fn or(&self, p:Parser<T, R>)->Parser<T, R>{
        let s:Parser<T, R> = self.clone();
        either(s, p)
    }
}

pub fn many<T:'static, R:'static, X:'static>(p:X)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, X:Parsec<T, R>+Clone {
    let p=p.clone();
    abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        either(many1(try(p.clone())), pack(Vec::<R>::new())).parse(state)
    })
}

pub fn many1<T:'static, R:'static, X:'static>(p:X)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, X:Parsec<T, R>+Monad<T, R>+Clone {
    p.clone().bind(abc!(move |state: &mut State<T>, x: R| -> Status<Vec<R>> {
        let mut rev = Vec::new();
        let tail = try!(many(p.clone()).parse(state));
        rev.push(x);
        rev.push_all(&tail);
        Ok(rev)
    }))
}

pub fn between<T:'static, B:'static, P:'static, E:'static, X:'static, Open:'static, Close:'static>
        (open:Open, close:Close, parsec:X)
        ->Parser<T, P>
where T:Clone, P:Clone, B:Clone, E:Clone, Open:Monad<T, B>+Clone, X:Parsec<T, P>+Clone,
        Close:Parsec<T, E>+Clone {
    let open = open.clone();
    let parsec = parsec.clone();
    let close = close.clone();
    abc!(move |state: &mut State<T>|->Status<P>{
        try!(open.parse(state));
        let re = parsec.parse(state);
        try!(close.parse(state));
        re
    })
}

pub fn otherwise<T:'static, R:'static, X:'static>(p:X, description:String)->Parser<T, R>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    abc!(move |state : &mut State<T>|->Status<R>{
        either(p.clone(), fail(description.clone()).clone()).parse(state)
    })
}

pub fn many_tail<T:'static, R:'static, Tl:'static, X:'static, Tail:'static>
    (p:X, tail:Tail)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Tl:Clone, X:Parsec<T, R>+Clone, Tail:Parsec<T, Tl>+Clone{
    abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let p = p.clone();
        let tail = tail.clone();
        many(p).over(tail).parse(state)
    })
}

pub fn many1_tail<T:'static, R:'static, Tl:'static, X:'static, Tail:'static>
    (p:X, tail:Tail)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Tl:Clone, X:Monad<T, R>+Clone, Tail:Parsec<T, Tl>+Clone{
    let p = p.clone();
    let tail = tail.clone();
    abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        many1(p.clone()).over(tail.clone()).parse(state)
    })
}

// We can use many/many1 as skip, but them more effective.
pub fn skip_many<T:'static, R:'static, X:'static>(p:X) ->Parser<T, Vec<R>>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    abc!(move |state: &mut State<T>|->Status<Vec<R>>{
        let p = try(p.clone());
        loop {
            let re = p.parse(state);
            if re.is_err() {
                return Ok(Vec::new());
            }
        }
    })
}

pub fn skip_many1<T:'static, R:'static, X:'static>(p:X) ->Parser<T, Vec<R>>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    abc!(move |state: &mut State<T>|->Status<Vec<R>>{
        let re = p.parse(state);
        if re.is_err() {
            return Err(re.err().unwrap());
        }
        skip_many(p.clone()).parse(state)
    })
}

pub fn sep_by<T:'static, Sp:'static, R:'static, Sep:'static, X:'static>(parsec:X, sep:Sep)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Sp:Clone, Sep:Parsec<T, Sp>+Clone, X:Parsec<T, R>+Clone {
    abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let s = try(sep.clone());
        let p = try(parsec.clone());
        either(sep_by1(p, s), pack(Vec::new())).parse(state)
    })
}

pub fn sep_by1<T:'static, Sp:'static, R:'static, Sep:'static, X:'static>(parsec:X, sep:Sep) ->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Sp:Clone, Sep:Parsec<T, Sp>+Clone, X:Parsec<T, R>+Clone {
    abc!(move |state: &mut State<T>|->Status<Vec<R>>{
        let parsec = parsec.clone();
        let x = parsec.parse(state);
        if x.is_err() {
            return Err(x.err().unwrap());
        }
        let mut rev = Vec::new();
        let head = x.ok().unwrap();
        let tail = sep_by(parsec.clone(), sep.clone()).parse(state);
        let data = tail.unwrap();
        rev.push(head);
        rev.push_all(&data);
        Ok(rev)
    })
}
