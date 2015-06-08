extern crate raskell;
use raskell::parsec::{VecState, State};
use std::sync::Arc;
use std::iter::FromIterator;


#[test]
fn it_works() {
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
