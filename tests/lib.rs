#![feature(collections)]

extern crate ruskell;
use ruskell::parsec::{VecState, State, Status, Parsec};
use ruskell::parsec::atom::{one, eof};
use ruskell::parsec::combinator::{either, bind, then};
use std::sync::Arc;
use std::iter::FromIterator;
use std::ops::Deref;


#[test]
fn state_works() {
    let mut state:VecState<char> = VecState::from_iter("abc".chars().into_iter());
    assert_eq!(state.next(), Some(Arc::new('a')));
    assert_eq!(state.pos(), 1);
    assert_eq!(state.next(), Some(Arc::new('b')));
    assert_eq!(state.pos(), 2);
    assert_eq!(state.next(), Some(Arc::new('c')));
    assert_eq!(state.pos(), 3);
    assert_eq!(state.next(), None);
    assert_eq!(state.pos(), 3);
}

#[test]
fn one_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let mut a = one::<char>(Arc::new('a'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('a'));
    let mut a = one::<char>(Arc::new('b'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('b'));
    let mut a = one::<char>(Arc::new('c'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('c'));
}

#[test]
fn one_end_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = &mut one(Arc::new('a'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('a'));
    let a = &mut one(Arc::new('b'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('b'));
    let a = &mut one(Arc::new('c'));
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('c'));
    let re = eof()(&mut state);
    assert!(re.is_ok());
}

#[test]
fn either_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one(Arc::new('a'));
    let b = one(Arc::new('b'));
    let e = &mut either(b, a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('a'));
}

#[test]
fn either_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one(Arc::new('a'));
    let b = one(Arc::new('b'));
    let e = &mut either(a, b);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('a'));
}

#[test]
fn bind_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one(Arc::new('a'));
    let exp = &mut bind(a, Box::new(|x:Arc<char>|->Parsec<char, Vec<Arc<char>>>{
            let x = x.clone();
            Box::new(move |state:&mut VecState<char>|->Status<Vec<Arc<char>>>{
                one(Arc::new('b'))(state).map(|y:Arc<char>| -> Arc<Vec<Arc<char>>>{
                    let x = x.clone();
                    let y = y.clone();
                    let mut res = Vec::new();
                    res.push(x);
                    res.push(y);
                    Arc::new(res)
                })
            })
        })).bind(Box::new(move |v:Arc<Vec<Arc<char>>>|->Parsec<char, Vec<Arc<char>>>{
            Box::new(move |state: &mut VecState<char>|->Status<Vec<Arc<char>>>{
                let v = v.clone();
                one(Arc::new('c'))(state).map(move |x:Arc<char>| -> Arc<Vec<Arc<char>>> {
                    let v = v.clone();
                    let x = x.clone();
                    let mut res = Vec::new();
                    res.push_all(v.deref());
                    res.push(x);
                    Arc::new(res)
                })
            })
        }));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = Arc::new((vec!['a', 'b', 'c']).into_iter().map(|x:char| Arc::new(x)).collect());
    assert_eq!(data, ver);
}

#[test]
fn then_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one(Arc::new('a'));
    let b = one(Arc::new('b'));
    let c = one(Arc::new('c'));
    let exp = &mut then(a, b).then(c);
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('c'));
}


#[test]
fn bind_then_over_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one(Arc::new('a'));
    let exp = &mut then(a, one(Arc::new('b'))).over(one(Arc::new('c'))).over(eof());
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, Arc::new('b'));
}
