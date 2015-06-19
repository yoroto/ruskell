use parsec::{VecState, State, Parsec, Status};
use std::marker::PhantomData;
//use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Try<T, R, P>{
    parsec : P,
    input: PhantomData<T>,
    output: PhantomData<R>,
}

impl<T, R, P> Try<T, R, P> where P:Parsec<T, R>, T:Clone  {
    pub fn new(p:P) -> Try<T, R, P> {
        Try{parsec:p, input:PhantomData, output:PhantomData}
    }
}

impl<T, R, P> Parsec<T, R> for Try<T, R, P> where P:Parsec<T, R>, T:Clone {
    fn parse(&self, state: &mut State<T>)->Status<R> {
        let pos = state.pos();
        let res = self.parsec.parse(state);
        if res.is_err() {
            state.seek_to(pos);
        }
        res
    }
}

impl<'a, T, R, P> FnOnce<(&'a mut VecState<T>, )> for Try<T, R, P> where P:Parsec<T, R>, T:Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, P> FnMut<(&'a mut VecState<T>, )> for Try<T, R, P> where P:Parsec<T, R>, T:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut VecState<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, P> Fn<(&'a mut VecState<T>, )> for Try<T, R, P> where P:Parsec<T, R>, T:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<R> {
        let (state, ) = args;
        self.parse(state)
    }
}

#[derive(Debug, Clone)]
pub struct Either<T, R, PX, PY>{
    x: PX,
    y: PY,
    input_type: PhantomData<T>,
    result_type: PhantomData<R>,
}

impl<T, R, PX, PY> Either<T, R, PX, PY>
where T:Clone, R:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{

    pub fn new(x:PX, y:PY) -> Either<T, R, PX, PY> {
        Either{x:x.clone(), y:y.clone(), input_type:PhantomData, result_type:PhantomData}
    }

    pub fn or<PZ>(&self, z:PZ)-> Either<T, R, Self, PZ> where PZ:Parsec<T, R>+Clone {
        let left = Either::new(self.x.clone(), self.y.clone());
        Either::new(left, z)
    }
}

impl<T, R, PX, PY> Parsec<T, R> for Either<T, R, PX, PY>
where T:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{
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

impl<'a, T, R, PX, PY> FnOnce<(&'a mut VecState<T>, )> for Either<T, R, PX, PY>
        where T:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, PX, PY> FnMut<(&'a mut VecState<T>, )> for Either<T, R, PX, PY>
        where T:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut VecState<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R, PX, PY> Fn<(&'a mut VecState<T>, )> for Either<T, R, PX, PY>
        where T:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{
    extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<R> {
        //self.call_once(args)
        let (state, ) = args;
        self.parse(state)
    }
}

pub fn either<T, R, PX, PY>(x: PX, y:PY)->Either<T, R, PX, PY>
    where T:Clone, R:Clone, PX:Parsec<T, R>+Clone, PY:Parsec<T, R>+Clone{
        Either::new(x, y)
}

//
// pub struct Bind<T:'static, C:'static, P:'static> {
//     parsec: Parsec<T, C>,
//     binder: Binder<T, C, P>,
// }
//
// pub fn bind<T:'static, C:'static, P:'static>(parsec:Parsec<T, C>, binder:Binder<T, C, P>)
//         -> Psc<Bind<T, C, P>> {
//     parsec!(Bind::new(parsec, binder))
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Bind<T, C, P> {
//     type Output = Status<P>;
//     extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<P> {
//         panic!("Not implement!");
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Bind<T, C, P> {
//     extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
//         let (state, ) = args;
//         (self.parsec)(state)
//                 .map(|x:Arc<C>| (self.binder)(x.clone()))
//                 .map(|p:Parsec<T, P>| p(state))
//                 .unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Bind<T, C, P> {
//     extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
//         let (state, ) = args;
//         (self.parsec)(state)
//                 .map(|x:Arc<C>| (self.binder)(x.clone()))
//                 .map(|p:Parsec<T, P>| p(state))
//                 .unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<T:'static, C:'static, P:'static> Bind<T, C, P>{
//     pub fn new(parsec:Parsec<T, C>, binder:Binder<T, C, P>)->Bind<T, C, P> {
//         Bind{
//             parsec:parsec.clone(),
//             binder:binder.clone(),
//         }
//     }
//
//     pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, P, Q>> {
//         let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
//         parsec!(Over{
//             prefix:parsec!(move |state: &mut VecState<T>| s(state)),
//             postfix:postfix.clone(),
//         })
//     }
//     pub fn bind<Q>(&self, binder:Binder<T, P, Q>) -> Psc<Bind<T, P, Q>> {
//         let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
//         parsec!(Bind{
//             parsec:parsec!(move |state: &mut VecState<T>| s(state)),
//             binder:binder.clone(),
//         })
//     }
//     pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, P, Q>> {
//         let s = parsec!(Bind::new(self.parsec.clone(), self.binder.clone()));
//         parsec!(Then{
//             prefix:parsec!(move |state: &mut VecState<T>| s(state)),
//             postfix:postfix.clone(),
//         })
//     }
// }
//
// // Type Continuation Then
// pub struct Then<T:'static, C:'static, P:'static> {
//     prefix: Parsec<T, C>,
//     postfix: Parsec<T, P>,
// }
//
// pub fn then<T:'static, C:'static, P:'static>(prefix:Parsec<T, C>,
//         postfix:Parsec<T, P>)->Psc<Then<T, C, P>> {
//     parsec!(Then::new(prefix, postfix))
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Then<T, C, P> {
//     type Output = Status<P>;
//     extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<P> {
//         panic!("Not implement!");
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Then<T, C, P> {
//     extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<P> {
//         let (state, ) = args;
//         (self.prefix)(state)
//                 .map(|_:Arc<C>| (self.postfix)(state))
//                 .unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Then<T, C, P> {
//     extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<P> {
//         let (state, ) = args;
//         (self.prefix)(state)
//                 .map(|_:Arc<C>| (self.postfix)(state))
//                 .unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<T:'static, C:'static, P:'static> Then<T, C, P>{
//     pub fn new(prefix:Parsec<T, C>, postfix:Parsec<T, P>)->Then<T, C, P> {
//         Then{
//             prefix:prefix.clone(),
//             postfix:postfix.clone(),
//         }
//     }
//
//     pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, P, Q>> {
//         let left = then(self.prefix.clone(), self.postfix.clone());
//         over(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
//     }
//     pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, P, Q>> {
//         let left = then(self.prefix.clone(), self.postfix.clone());
//         then(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
//     }
//     pub fn bind<Q>(&self, binder:Binder<T, P, Q>) -> Psc<Bind<T, P, Q>> {
//         let left = then(self.prefix.clone(), self.postfix.clone());
//         bind(parsec!(move |state:&mut VecState<T>|left(state)), binder.clone())
//     }
// }
//
// // Type Continuation Then
// pub struct Over<T:'static, C:'static, P:'static> {
//     prefix: Parsec<T, C>,
//     postfix: Parsec<T, P>,
// }
//
// pub fn over<T:'static, C:'static, P:'static>(prefix:Parsec<T, C>,
//             postfix:Parsec<T, P>)->Psc<Over<T, C, P>> {
//     parsec!(Over::new(prefix, postfix))
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Over<T, C, P> {
//     type Output = Status<C>;
//     extern "rust-call" fn call_once(self, _: (&'a mut VecState<T>, )) -> Status<C> {
//         panic!("Not implement!");
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> FnMut<(&'a mut VecState<T>, )> for Over<T, C, P> {
//     extern "rust-call" fn call_mut(&mut self, args: (&'a mut VecState<T>, )) -> Status<C> {
//         let (state, ) = args;
//         (self.prefix)(state)
//             .map(|x:Arc<C>|->Status<C>{
//                 (self.postfix)(state).map(|_:Arc<P>| x.clone())
//             }).unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<'a, T:'static, C:'static, P:'static> Fn<(&'a mut VecState<T>, )> for Over<T, C, P> {
//     extern "rust-call" fn call(&self, args: (&'a mut VecState<T>, )) -> Status<C> {
//         let (state, ) = args;
//         (self.prefix)(state)
//             .map(|x:Arc<C>|->Status<C>{
//                 (self.postfix)(state).map(|_:Arc<P>| x.clone())
//             }).unwrap_or_else(|err:SimpleError| Err(err))
//     }
// }
//
// impl<T:'static, C:'static, P:'static> Over<T, C, P>{
//     pub fn new(prefix:Parsec<T, C>, postfix:Parsec<T, P>)->Over<T, C, P> {
//         Over{
//             prefix:prefix,
//             postfix:postfix,
//         }
//     }
//     pub fn over<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Over<T, C, Q>> {
//         let left = over(self.prefix.clone(), self.postfix.clone());
//         over(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
//     }
//     pub fn then<Q>(&self, postfix:Parsec<T, Q>) -> Psc<Then<T, C, Q>> {
//         let left = over(self.prefix.clone(), self.postfix.clone());
//         then(parsec!(move |state:&mut VecState<T>|left(state)), postfix.clone())
//     }
//     pub fn bind<Q>(&self, binder:Binder<T, C, Q>) -> Psc<Bind<T, C, Q>> {
//         let left = over(self.prefix.clone(), self.postfix.clone());
//         bind(parsec!(move |state:&mut VecState<T>|left(state)), binder.clone())
//     }
// }
//
// pub fn many<T:'static, R:'static, P:Fn(&mut VecState<T>)->Status<R>+?Sized+'static>(p: Arc<Box<P>>)
//         -> Parsec<T, Vec<Arc<R>>> {
//     let re = Box::new(Either::new(many1(try(p)), pack(Arc::new(Vec::new()))));
//     Arc::new(re as Box<Parser<T, Vec<Arc<R>>>>)
// }
//
// pub fn many1<T:'static, R:'static, P:Fn(&mut VecState<T>)->Status<R>+?Sized+'static>(p: Arc<Box<P>>)
//         -> Parsec<T, Vec<Arc<R>>> {
//     let p = p.clone();
//     parsec!(move |state: &mut VecState<T>|->Status<Vec<Arc<R>>>{
//         let p = p.clone();
//         p(state).map(move |x:Arc<R>|->Status<Vec<Arc<R>>>{
//             let psc = many(p.clone());
//             let follow = psc(state);
//             let mut v = Vec::new();
//             v.push(x.clone());
//             v.push_all(follow.unwrap().deref());
//             Ok(Arc::new(v))
//         }).unwrap_or_else(|err:SimpleError| Err(err))
//     })
// }
