extern crate raskell;
use raskell::parsec::{VecState, State};
use raskell::parsec::atom::{one, eof};
use std::sync::Arc;
use std::iter::FromIterator;


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
    let mut state:VecState<char> = VecState::from_iter("abc".chars().into_iter());
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
}

#[test]
fn one_end_test_0() {
    let mut state:VecState<char> = VecState::from_iter("abc".chars().into_iter());
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
    let re = eof(&mut state);
    assert!(re.is_ok());
}
