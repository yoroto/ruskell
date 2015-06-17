use parsec::{VecState, State, SimpleError, Parsec, Parser, Psc, Status, Binder};
use std::sync::Arc;
use std::ops::Deref;

pub fn pack<T:'static, R:'static>(data:Arc<R>) -> Parsec<T, R> {
    parsec!(move |_:&mut VecState<T>|-> Status<R> {
        let data=data.clone();
        Ok(data)
    })
}

pub fn try<T:'static, R:'static>(parsec:Parsec<T, R>) -> Parsec<T, R> {
    parsec!(move |state:&mut VecState<T>|-> Status<R> {
        let p = parsec.clone();
        let pos = state.pos();
        let val = p(state);
        if val.is_err() {
            state.seek_to(pos);
        }
        val
    })
}

pub fn fail<T:'static>(msg: String)->Parsec<T, ()> {
    parsec!(move |state:&mut VecState<T>|-> Status<()> {
        Err(SimpleError::new(state.pos(), msg.clone()))
    })
}

pub struct Either<T:'static, R:'static>{
    x: Parsec<T, R>,
    y: Parsec<T, R>,
}

pub fn either<T:'static, R:'static>(x: Parsec<T, R>, y: Parsec<T, R>)-> Psc<Either<T, R>> {
    parsec!(Either::new(x.clone(), y.clone()))
}

impl<'a, T:'static, R:'static> FnOnce<(&'a mut VecState<T>, )> for Either<T, R> {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<R> {
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

impl<'a, T:'static, R:'static> FnMut<(&'a mut VecState<T>, )> for Either<T, R> {
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

impl<'a, T:'static, R:'static> Fn<(&'a mut VecState<T>, )> for Either<T, R> {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<R> {
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
    pub fn new(x: Parsec<T, R>, y:Parsec<T, R>)->Either<T, R> {
        Either{
            x: x.clone(),
            y: y.clone(),
        }
    }

    pub fn or(&self, p:Parsec<T, R>) -> Psc<Self> {
        let right = Either::new(self.y.clone(), p.clone());
        either(self.x.clone(), Arc::new(Box::new(move |state:&mut VecState<T>|right(state))))
    }
}

// Type Continuation Then
pub struct Bind<T:'static, C:'static, P:'static> {
    parsec: Parsec<T, C>,
    binder: Binder<T, C, P>,
}

pub fn bind<T:'static, C:'static, P:'static>(parsec:Parsec<T, C>, binder:Binder<T, C, P>)
        -> Psc<Bind<T, C, P>> {
    parsec!(Bind::new(parsec, binder))
}

impl<'a, T:'static, C:'static, P:'static> FnOnce<(&'a mut VecState<T>, )> for Bind<T, C, P> {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Bind<T, C, P> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Bind<T, C, P> {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static> Bind<T, C, P>{
    pub fn new(parsec:Parsec<T, C>, binder:Binder<T, C, P>)->Bind<T, C, P> {
        Bind{
            parsec:parsec.clone(),
            binder:binder.clone(),
        }
    }

    pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, P, Q>> {
        let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
        parsec!(Over{
            prefix:parsec!(move |state: &mut VecState<T>| s(state)),
            postfix:postfix.clone(),
        })
    }
    pub fn bind<Q>(&self, binder:Binder<T, P, Q>) -> Psc<Bind<T, P, Q>> {
        let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
        parsec!(Bind{
            parsec:parsec!(move |state: &mut VecState<T>| s(state)),
            binder:binder.clone(),
        })
    }
    pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, P, Q>> {
        let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
        parsec!(Then{
            prefix:parsec!(move |state: &mut VecState<T>| s(state)),
            postfix:postfix.clone(),
        })
    }
}

// Type Continuation Then
pub struct Then<T:'static, C:'static, P:'static> {
    prefix: Parsec<T, C>,
    postfix: Parsec<T, P>,
}

pub fn then<T:'static, C:'static, P:'static>(prefix:Parsec<T, C>,
        postfix:Parsec<T, P>)->Psc<Then<T, C, P>> {
    parsec!(Then::new(prefix, postfix))
}

impl<'a, T:'static, C:'static, P:'static> FnOnce<(&'a mut VecState<T>, )> for Then<T, C, P> {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Then<T, C, P> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Then<T, C, P> {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static> Then<T, C, P>{
    pub fn new(prefix:Parsec<T, C>, postfix:Parsec<T, P>)->Then<T, C, P> {
        Then{
            prefix:prefix.clone(),
            postfix:postfix.clone(),
        }
    }

    pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, P, Q>> {
        let left = then(self.prefix.clone(), self.postfix.clone());
        over(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, P, Q>> {
        let left = then(self.prefix.clone(), self.postfix.clone());
        then(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn bind<Q>(&self, binder:Binder<T, P, Q>) -> Psc<Bind<T, P, Q>> {
        let left = then(self.prefix.clone(), self.postfix.clone());
        bind(parsec!(move |state:&mut VecState<T>|left(state)), binder.clone())
    }
}

// Type Continuation Then
pub struct Over<T:'static, C:'static, P:'static> {
    prefix: Parsec<T, C>,
    postfix: Parsec<T, P>,
}

pub fn over<T:'static, C:'static, P:'static>(prefix:Parsec<T, C>,
            postfix:Parsec<T, P>)->Psc<Over<T, C, P>> {
    parsec!(Over::new(prefix, postfix))
}

impl<'a, T:'static, C:'static, P:'static> FnOnce<(&'a mut VecState<T>, )> for Over<T, C, P> {
    type Output = Status<C>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|x:Arc<C>|->Status<C>{
                    (self.postfix)(state).map(|_:Arc<P>| x.clone())
                }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Over<T, C, P> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
            .map(|x:Arc<C>|->Status<C>{
                (self.postfix)(state).map(|_:Arc<P>| x.clone())
            }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Over<T, C, P> {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
            .map(|x:Arc<C>|->Status<C>{
                (self.postfix)(state).map(|_:Arc<P>| x.clone())
            }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static> Over<T, C, P>{
    pub fn new(prefix:Parsec<T, C>, postfix:Parsec<T, P>)->Over<T, C, P> {
        Over{
            prefix:prefix,
            postfix:postfix,
        }
    }
    pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, C, Q>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        over(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, C, Q>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        then(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn bind<Q>(&self, binder:Binder<T, C, Q>) -> Psc<Bind<T, C, Q>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        bind(parsec!(move |state:&mut VecState<T>|left(state)), binder.clone())
    }
}


pub fn many<T:'static, R:'static>(p: Parsec<T, R>) -> Parsec<T, Vec<Arc<R>>> {
    // let p = p.clone();
    // parsec!(move |state:&mut VecState<T>|->Status<Vec<Arc<R>>>{
    //     let p = p.clone();
    //     let mut rev:Vec<Arc<R>> = Vec::new();
    //     loop {
    //         let re = try(p)(state);
    //         if re.is_err() {
    //             break;
    //         } else {
    //             rev.push(re.unwrap());
    //         }
    //     }
    //     Ok(Arc::new(rev))
    // })
    let re = Box::new(Either::new(many1(try(p)), pack(Arc::new(Vec::new()))));
    Arc::new(re as Box<Parser<T, Vec<Arc<R>>>>)
}

pub fn many1<T:'static, R:'static>(p: Parsec<T, R>) -> Parsec<T, Vec<Arc<R>>> {
    // parsec!(move |state: &mut VecState<T>|->Status<Vec<Arc<R>>>{
    //     let head = p(state);
    //     if head.is_err() {
    //         return Err(head.err().unwrap());
    //     }
    //     let mut rev:Vec<Arc<R>> = vec![head.unwrap()];
    //     loop {
    //         let re = try(p.clone())(state);
    //         if re.is_err() {
    //             break;
    //         } else {
    //             rev.push(re.unwrap());
    //         }
    //     }
    //     Ok(Arc::new(rev))
    // })
    let p = p.clone();
    parsec!(move |state: &mut VecState<T>|->Status<Vec<Arc<R>>>{
        let p = p.clone();
        p(state).map(move |x:Arc<R>|->Status<Vec<Arc<R>>>{
            let psc = many(p.clone());
            let follow = psc(state);
            let mut v = Vec::new();
            v.push_all(follow.unwrap().deref());
            v.push(x.clone());
            Ok(Arc::new(v))
        }).unwrap_or_else(|err:SimpleError| Err(err))
    })
}
