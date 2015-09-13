use std::sync::Arc;
use std::boxed::Box;
use std::clone::Clone;

#[derive(Clone)]
pub enum Mu<T> {
    Roll(Arc<Box<Fn(Mu<T>)->T>>),
}

pub fn unroll<T>(Mu::Roll(f): Mu<T>) -> Arc<Box<Fn(Mu<T>)->T>> {f.clone()}

pub type Func<A, B> = Arc<Box<Fn(A)->B>>;
pub type RecFunc<A, B> = Arc<Box<Fn(Func<A, B>) -> Func<A, B>>>;

pub fn y<A:'static, B:'static>(f: RecFunc<A, B>) -> Func<A, B> {
    let g:Arc<Box<Fn(Mu<Func<A, B>>)->Func<A, B>>> = abc!(move |x : Mu<Func<A, B>>| -> Func<A, B> {
        let f = f.clone();
        abc!(move |a:A| -> B {
            let f = f.clone();
            f(unroll(x.clone())(x.clone()))(a)
        })
    });
    g(Mu::Roll(g.clone()))
}
