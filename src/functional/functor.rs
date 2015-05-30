use std::result::Result;
use std::option::Option;

pub trait Functor<A, B, F>{
    type Output;
    fn fmap(&self, f: &F ) -> <Self as Functor<A, B, F>>::Output;
}

impl<A, B, F> Functor<A, B, F> for Vec<A>
where F:Fn(&A)->B {
    type Output=Vec<B>;
    fn fmap(&self, f:&F) -> Self::Output {
        let mut re = Vec::with_capacity(self.len());
        for data in self {
            re.push(f(data));
        }
        re
    }
}

#[test]
fn vec_functor_test0() {
    let source = vec![0, 1, 2, 3, 4];
    let f = |x:&i32| x*2;
    let data = source.fmap(&f);
    assert_eq!(data, vec![0, 2, 4, 6, 8]);
}

#[test]
fn vec_functor_test1() {
    let source = vec![0, 1, 2, 3, 4];
    let f:Box<Fn(i32)->i32> = Box::new(|x:i32| x*2);
    let data = source.into_iter().map(|x:i32|f(x)).collect::<Vec<_>>();
    assert_eq!(data, vec![0, 2, 4, 6, 8]);
}

impl<T:Copy, B:Copy, F, E:Copy> Functor<T, B, F> for Result<T, E>
where F: Fn(T)->B, E:Copy {
    type Output=Result<B, E>;
    fn fmap(&self, f:&F) -> Self::Output {
        self.map(f)
    }
}

#[test]
fn result_functor_test_ok() {
    let source:Result<i32, i32> = Ok(25);
    let data = source.fmap(&|x|x/5);
    let check:Result<i32, i32> = Ok(5);
    assert_eq!(data, check);
}

#[test]
fn result_functor_test_error() {
    let nothing = "Nothing".to_string();
    let source:Result<i32, &str> = Err(&nothing);
    let data = source.fmap(&|x|x/5);
    let check:Result<i32, &str> = Err(&nothing);
    assert_eq!(data, check);
}

impl<T:Copy, B:Copy, F> Functor<T, B, F> for Option<T>
where F: Fn(T)->B {
    type Output=Option<B>;
    fn fmap(&self, f:&F) -> Self::Output {
        self.map(f)
    }
}

#[test]
fn option_functor_test_ok() {
    let source:Option<i32> = Some(25);
    let data = source.fmap(&|x|x/5);
    let check:Option<i32> = Some(5);
    assert_eq!(data, check);
}

#[test]
fn result_functor_test_none() {
    let source:Option<i32> = None;
    let data = source.fmap(&|x|x/5);
    assert_eq!(data, source);
}
