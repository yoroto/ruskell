use parsec::{VecState, State, SimpleError, Parsec, Parser, Psc, Status, Binder};
use std::sync::Arc;
use std::ops::Deref;
use std::marker::{Sized, PhantomData};
//use std::fmt::{Debug, Display};

pub fn pack<T:'static, R:'static>(data:Arc<R>) -> Parsec<T, R> {
    parsec!(move |_:&mut VecState<T>|-> Status<R> {
        let data=data.clone();
        Ok(data)
    })
}

pub fn try<T:'static, R:'static, P:Fn(&mut VecState<T>)->Status<R>+?Sized+'static>(parsec:Arc<Box<P>>) -> Parsec<T, R> {
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
pub struct Bind<T:'static, C:'static, P:'static, PTC:'static+?Sized, BCP:'static+?Sized> {
    parsec: Arc<Box<PTC>>,
    binder: Arc<Box<BCP>>,
    element_type: PhantomData<T>,
    continuation_type: PhantomData<C>,
    pass_type: PhantomData<P>,
}

pub fn bind<T:'static, C:'static, P:'static, PTC:?Sized+'static, BCP:?Sized+'static>
        (parsec:Arc<Box<PTC>>, binder:Arc<Box<BCP>>) -> Psc<Bind<T, C, P, PTC, BCP>>
        where PTC:Fn(&mut VecState<T>)->Status<C>, BCP:Fn(Arc<C>)->Parsec<T, P> {
    parsec!(Bind::new(parsec, binder))
}

impl<'a, T:'static, C:'static, P:'static, PTC:?Sized+'static, BCP:?Sized+'static>
            FnOnce<(&'a mut VecState<T>, )> for Bind<T, C, P, PTC, BCP>
            where PTC:Fn(&mut VecState<T>)->Status<C>, BCP:Fn(Arc<C>)->Parsec<T, P> {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:?Sized+'static, BCP:?Sized+'static>
            FnMut<(&'a mut VecState<T>, )> for Bind<T, C, P, PTC, BCP>
            where PTC:Fn(&mut VecState<T>)->Status<C>, BCP:Fn(Arc<C>)->Parsec<T, P> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:?Sized+'static, BCP:?Sized+'static>
            Fn<(&'a mut VecState<T>, )> for Bind<T, C, P, PTC, BCP>
            where PTC:Fn(&mut VecState<T>)->Status<C>, BCP:Fn(Arc<C>)->Parsec<T, P>{
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.parsec)(state)
                .map(|x:Arc<C>| (self.binder)(x.clone()))
                .map(|p:Parsec<T, P>| p(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static, PTC:?Sized, BCP:?Sized> Bind<T, C, P, PTC, BCP>
        where PTC:Fn(&mut VecState<T>)->Status<C>+'static,
            BCP:Fn(Arc<C>)->Parsec<T, P>+'static{
    pub fn new(parsec:Arc<Box<PTC>>, binder:Arc<Box<BCP>>)->Bind<T, C, P, PTC, BCP> {
        Bind{
            parsec:parsec.clone(),
            binder:binder.clone(),
            element_type: PhantomData,
            continuation_type: PhantomData,
            pass_type: PhantomData,
        }
    }

    pub fn over<Q, PTP:?Sized+'static, PTQ:?Sized+'static,>
            (&self, postfix:Arc<Box<PTQ>>) -> Psc<Over<T, P, Q, PTP, PTQ>>
            where PTP:Fn(&mut VecState<T>)->Status<P>, PTQ:Fn(&mut VecState<T>)->Status<Q> {
        let left = bind(self.parsec.clone(), self.binder.clone());
        over(left.clone(), postfix.clone())
    }
    pub fn bind<Q, PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static,
                BPQ:Fn(Arc<P>)->Parsec<T, Q>+?Sized+'static>(&self, binder:Arc<Box<BPQ>>)
            -> Psc<Bind<T, P, Q, PTP, BPQ>> {
        let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
        bind(s.clone(), binder.clone())
    }
    pub fn then<Q, PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static,
                PTQ:Fn(&mut VecState<T>)->Status<Q>+?Sized+'static>(&self, postfix:Arc<Box<PTQ>>)
            -> Psc<Then<T, P, Q, PTP, PTQ>> {
        let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
        parsec!(Then::new(parsec!(move |state: &mut VecState<T>| s(state)), postfix.clone()))
    }
}

// Type Continuation Then
pub struct Then<T:'static, C:'static, P:'static, PTC:'static+?Sized, PTP:'static+?Sized> {
    prefix: Arc<Box<PTC>>,
    postfix: Arc<Box<PTP>>,
    element_type: PhantomData<T>,
    continuation_type: PhantomData<C>,
    pass_type: PhantomData<P>,
}

pub fn then<T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static>(prefix:Parsec<T, C>,
        postfix:Parsec<T, P>)->Psc<Then<T, C, P, PTC, PTP>> {
    parsec!(Then::new(prefix, postfix))
}

impl<'a, T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> FnOnce<(&'a mut VecState<T>, )>
        for Then<T, C, P, PTC, PTP> {
    type Output = Status<P>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> FnMut<(&'a mut VecState<T>, )>
        for Then<T, C, P, PTC, PTP> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:?Sized+'static, PTP:?Sized+'static>
        Fn<(&'a mut VecState<T>, )> for Then<T, C, P, PTC, PTP>
        where PTC:Fn(&mut VecState<T>)->Status<C>, PTP:Fn(&mut VecState<T>)->Status<P>{
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|_:Arc<C>| (self.postfix)(state))
                .unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static, PTC:?Sized+'static, PTP:?Sized+'static> Then<T, C, P, PTC, PTP>
    where PTC:Fn(&mut VecState<T>)->Status<C>, PTP:Fn(&mut VecState<T>)->Status<P> {

    pub fn new(prefix:Arc<Box<PTC>>, postfix:Arc<Box<PTP>>)->Then<T, C, P, PTC, PTP> {
        Then{
            prefix:prefix.clone(),
            postfix:postfix.clone(),
            element_type: PhantomData,
            continuation_type: PhantomData,
            pass_type: PhantomData,
        }
    }

    pub fn over<Q, PTQ:?Sized+'static>(&self, postfix:Arc<Box<PTQ>>) -> Psc<Over<T, P, Q, PTP, PTQ>>
            where PTQ:Fn(&mut VecState<T>)->Status<Q>{
        let left = then(self.prefix.clone(), self.postfix.clone());
        over(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn then<Q, PTQ:Fn(&mut VecState<T>)->Status<Q>+?Sized+'static>(&self, postfix:Arc<Box<PTQ>>)
            -> Psc<Then<T, P, Q, PTP, PTQ>> {
        let left = then(self.prefix.clone(), self.postfix.clone());
        then(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
    }
    pub fn bind<Q, BPQ:Fn(Arc<P>)->Parsec<T, Q>+?Sized+'static>(&self, binder:Binder<T, P, Q>)
            -> Psc<Bind<T, P, Q, PTP, BPQ>> {
        let left = then(self.prefix.clone(), self.postfix.clone());
        bind(parsec!(move |state:&mut VecState<T>|left(state)), binder.clone())
    }
}

// Type Continuation Then
pub struct Over<T:'static, C:'static, P:'static, PTC:?Sized+'static, PTP:?Sized+'static> {
    prefix: Arc<Box<PTC>>,
    postfix: Arc<Box<PTP>>,
    element_type: PhantomData<T>,
    continuation_type: PhantomData<C>,
    pass_type: PhantomData<P>,
}

pub fn over<T:'static, C:'static, P:'static, PTC:?Sized+'static, PTP:?Sized+'static>
            (prefix:Arc<Box<PTC>>, postfix:Arc<Box<PTP>>) ->Psc<Over<T, C, P, PTC, PTP>>
            where PTC:Fn(&mut VecState<T>)->Status<C>, PTP:Fn(&mut VecState<T>)->Status<P>{
    parsec!(Over::new(prefix, postfix))
}

impl<'a, T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> FnOnce<(&'a mut VecState<T>, )>
        for Over<T, C, P, PTC, PTP> {
    type Output = Status<C>;
    extern "rust-call" fn call_once(self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
                .map(|x:Arc<C>|->Status<C>{
                    (self.postfix)(state).map(|_:Arc<P>| x.clone())
                }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> FnMut<(&'a mut VecState<T>, )>
        for Over<T, C, P, PTC, PTP> {
    extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
            .map(|x:Arc<C>|->Status<C>{
                (self.postfix)(state).map(|_:Arc<P>| x.clone())
            }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<'a, T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> Fn<(&'a mut VecState<T>, )>
        for Over<T, C, P, PTC, PTP> {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<C> {
        let (state, ) = args;
        (self.prefix)(state)
            .map(|x:Arc<C>|->Status<C>{
                (self.postfix)(state).map(|_:Arc<P>| x.clone())
            }).unwrap_or_else(|err:SimpleError| Err(err))
    }
}

impl<T:'static, C:'static, P:'static, PTC:Fn(&mut VecState<T>)->Status<C>+?Sized+'static,
            PTP:Fn(&mut VecState<T>)->Status<P>+?Sized+'static> Over<T, C, P, PTC, PTP>{
    pub fn new(prefix:Arc<Box<PTC>>, postfix:Arc<Box<PTP>>)->Over<T, C, P, PTC, PTP> {
        Over{
            prefix:prefix,
            postfix:postfix,
            element_type: PhantomData,
            continuation_type: PhantomData,
            pass_type: PhantomData,
        }
    }
    pub fn over<Q, PTQ:Fn(&mut VecState<T>)->Status<Q>+?Sized+'static>(&self, postfix:Arc<Box<PTQ>>)
            -> Psc<Over<T, C, Q, PTC, PTQ>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        over(left.clone(), postfix.clone())
    }
    pub fn then<Q, PTQ:Fn(&mut VecState<T>)->Status<Q>+?Sized+'static>(&self, postfix:Arc<Box<PTQ>>)
            -> Psc<Then<T, C, Q, PTC, PTQ>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        then(left.clone(), postfix.clone())
    }
    pub fn bind<Q, BCQ:Fn(Arc<C>)->Parsec<T, Q>+?Sized+'static>(&self, binder:Arc<Box<BCQ>>)
            -> Psc<Bind<T, C, Q, PTC, BCQ>> {
        let left = over(self.prefix.clone(), self.postfix.clone());
        bind(left.clone(), binder.clone())
    }
}

pub fn between<T:'static, Start:'static, End:'static, R:'static,
                PS:?Sized+'static, PE:?Sized+'static, P:?Sized+'static>
            (start: Arc<Box<PS>>, p:Arc<Box<P>>, end: Arc<Box<PE>>) -> Arc<Box<P>>
            where PS:Fn(&mut VecState<T>)->Status<Start>, PE:Fn(&mut VecState<T>)->Status<End>,
                P:Fn(&mut VecState<T>)->Status<R> {
    then(start, p).over(end)
}

pub fn many<T:'static, R:'static, P:Fn(&mut VecState<T>)->Status<R>+?Sized+'static>(p: Arc<Box<P>>)
        -> Parsec<T, Vec<Arc<R>>> {
    let re = Box::new(Either::new(many1(try(p)), pack(Arc::new(Vec::new()))));
    Arc::new(re as Box<Parser<T, Vec<Arc<R>>>>)
}

pub fn many1<T:'static, R:'static, P:Fn(&mut VecState<T>)->Status<R>+?Sized+'static>(p: Arc<Box<P>>)
        -> Parsec<T, Vec<Arc<R>>> {
    let p = p.clone();
    parsec!(move |state: &mut VecState<T>|->Status<Vec<Arc<R>>>{
        let p = p.clone();
        p(state).map(move |x:Arc<R>|->Status<Vec<Arc<R>>>{
            let psc = many(p.clone());
            let follow = psc(state);
            let mut v = Vec::new();
            v.push(x.clone());
            v.push_all(follow.unwrap().deref());
            Ok(Arc::new(v))
        }).unwrap_or_else(|err:SimpleError| Err(err))
    })
}
