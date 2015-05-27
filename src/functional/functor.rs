use std::vec::Vec;
use std::result::Result;
use std::option::Option;

pub trait Functor<A, B, F> {
    type Output;
    fn fmap(self, f: F ) -> <Self as Functor<A, B, F>>::Output;
}

impl<A, B, F> Functor<A, B, F> for Vec<A>
where F:FnMut(A)->B {
    type Output=Vec<B>;
    fn fmap(self, f:F) -> Self::Output {
        self.into_iter().map(f).collect::<Self::Output>()
    }
}

#[test]
fn vec_functor_test0() {
    let source = vec![0, 1, 2, 3, 4];
    let f = |x:i32| x*2;
    let data = source.fmap(f);
    assert_eq!(data, vec![0, 2, 4, 6, 8]);
}

impl<T, B, F, E> Functor<T, B, F> for Result<T, E>
where F: FnOnce(T)->B {
    type Output=Result<B, E>;
    fn fmap(self, f:F) -> Self::Output {
        self.map(f)
    }
}

#[test]
fn result_functor_test_ok() {
    let source:Result<i32, String> = Ok(25);
    let data = source.fmap(|x|x/5);
    let check:Result<i32, String> = Ok(5);
    assert_eq!(data, check);
}

#[test]
fn result_functor_test_error() {
    let source:Result<i32, String> = Err("Nothing".to_string());
    let data = source.fmap(|x|x/5);
    let check:Result<i32, String> = Err("Nothing".to_string());
    assert_eq!(data, check);
}

impl<T, B, F> Functor<T, B, F> for Option<T>
where F: FnOnce(T)->B {
    type Output=Option<B>;
    fn fmap(self, f:F) -> Self::Output {
        self.map(f)
    }
}

#[test]
fn option_functor_test_ok() {
    let source:Option<i32> = Some(25);
    let data = source.fmap(|x|x/5);
    let check:Option<i32> = Some(5);
    assert_eq!(data, check);
}

#[test]
fn result_functor_test_none() {
    let source:Option<i32> = None;
    let data = source.fmap(|x|x/5);
    assert_eq!(data, source);
}
