use parsec::{State, Parsec, Status, Monad, Parser, parser};
use parsec::atom::{pack, fail};
use std::fmt::{Debug, Formatter};
use std::fmt;
use std::sync::Arc;
use std::marker::PhantomData;

pub fn try<T:'static, R:'static, X:'static>(p:X)->Parser<T, R>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state: &mut State<T>|->Status<R>{
        let pos = state.pos();
        let res = p.parse(state);
        if res.is_err() {
            state.seek_to(pos);
        }
        res
    }))
}

pub struct Either<T, R, X, Y>{
    x: X,
    y: Y,
    input: PhantomData<T>,
    output: PhantomData<R>,
}

impl<T:'static, R:'static, X, Y> Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    pub fn new(x:X, y:Y) -> Either<T, R, X, Y> {
        Either{x:x.clone(), y:y.clone(), input:PhantomData, output:PhantomData}
    }

    pub fn or<Z>(&self, z:Z)-> Either<T, R, Self, Z> where Z:Parsec<T, R>+Clone{
        let left = Either::new(self.x.clone(), self.y.clone());
        Either::new(left, z.clone())
    }
}

impl<T, R, X, Y> Parsec<T, R> for Either<T, R, X, Y>
where  T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    fn parse(&self, state:&mut State<T>)->Status<R> {
        let pos = state.pos();
        let val = self.x.parse(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                self.y.parse(state)
            } else {
                val
            }
        }
    }
}

impl<'a, T, R, X, Y> FnOnce<(&'a mut State<T>, )> for Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, X, Y> FnMut<(&'a mut State<T>, )> for Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, X, Y> Fn<(&'a mut State<T>, )> for Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        //self.call_once(args)
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R, X, Y> Clone for Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    fn clone(&self)->Self {
        Either{x:self.x.clone(), y:self.y.clone(), input:PhantomData, output:PhantomData}
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x.clone();
        self.y = source.y.clone();
    }
}

impl<T, R, X, Y> Debug for Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone {
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<either parsec>".fmt(formatter)
    }
}

impl<T:'static, R:'static, X:'static, Y:'static> Monad<T, R> for Either<T, R, X, Y>
where  T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone{}

pub fn either<T:'static, R:'static, X:'static, Y:'static>(x:X, y:Y)->Either<T, R, X, Y>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone, Y:Parsec<T, R>+Clone{
    Either::new(x, y)
}

pub fn many<T:'static, R:'static, X:'static>(p:X)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, X:Parsec<T, R>+Clone {
    parser(abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let p=p.clone();
        either(many1(try(p)), pack(Vec::new())).parse(state)
    }))
}

pub fn many1<T:'static, R:'static, X:'static>(p:X)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, X:Monad<T, R>+Clone {
    p.clone().bind(abc!(move |state: &mut State<T>, x: R| -> Status<Vec<R>> {
        let mut rev = Vec::new();
        let tail = many(p.clone()).parse(state);
        let data = tail.unwrap();
        rev.push(x);
        rev.push_all(&data);
        Ok(rev)
    }))
}

pub fn between<T:'static, B:'static, P:'static, E:'static, X:'static, Begin:'static, End:'static>
        (begin:Begin, parsec:X, end:End)
        ->Parser<T, P>
where T:Clone, P:Clone, B:Clone, E:Clone, Begin:Monad<T, B>+Clone, X:Parsec<T, P>+Clone,
        End:Parsec<T, E>+Clone {
    parser(abc!(move |state: &mut State<T>|->Status<P>{
        let begin = begin.clone();
        let parsec = parsec.clone();
        let end = end.clone();
        begin.then(parsec).over(end).parse(state)
    }))
}

pub fn otherwise<T:'static, R:'static, X:'static>(p:X, description:String)->Parser<T, R>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state : &mut State<T>|->Status<R>{
        either(p.clone(), fail(description.clone()).clone()).parse(state)
    }))
}

pub fn many_tail<T:'static, R:'static, Tl:'static, X:'static, Tail:'static>
    (p:X, tail:Tail)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Tl:Clone, X:Parsec<T, R>+Clone, Tail:Parsec<T, Tl>+Clone{
    parser(abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let p = p.clone();
        let tail = tail.clone();
        many(p).over(tail).parse(state)
    }))
}

pub fn many1_tail<T:'static, R:'static, Tl:'static, X:'static, Tail:'static>
    (p:X, tail:Tail)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Tl:Clone, X:Monad<T, R>+Clone, Tail:Parsec<T, Tl>+Clone{
    parser(abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let p = p.clone();
        let tail = tail.clone();
        many1(p).over(tail).parse(state)
    }))
}

// We can use many/many1 as skip, but them more effective.
pub fn skip_many<T:'static, R:'static, X:'static>(p:X) ->Parser<T, Vec<R>>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state: &mut State<T>|->Status<Vec<R>>{
        let p = try(p.clone());
        loop {
            let re = p.parse(state);
            if re.is_err() {
                return Ok(Vec::new());
            }
        }
    }))
}

pub fn skip_many1<T:'static, R:'static, X:'static>(p:X) ->Parser<T, Vec<R>>
where T:Clone, R:Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state: &mut State<T>|->Status<Vec<R>>{
        let re = p.parse(state);
        if re.is_err() {
            return Err(re.err().unwrap());
        }
        skip_many(p.clone()).parse(state)
    }))
}

pub fn sep_by<T:'static, Sp:'static, R:'static, Sep:'static, X:'static>(parsec:X, sep:Sep)->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Sp:Clone, Sep:Parsec<T, Sp>+Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state:&mut State<T>|->Status<Vec<R>>{
        let s = try(sep.clone());
        let p = try(parsec.clone());
        either(sep_by1(p, s), pack(Vec::new())).parse(state)
    }))
}

pub fn sep_by1<T:'static, Sp:'static, R:'static, Sep:'static, X:'static>(parsec:X, sep:Sep) ->Parser<T, Vec<R>>
where T:Clone, R:Clone+Debug, Sp:Clone, Sep:Parsec<T, Sp>+Clone, X:Parsec<T, R>+Clone {
    parser(abc!(move |state: &mut State<T>|->Status<Vec<R>>{
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
    }))
}
