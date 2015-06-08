extern crate raskell;
use raskell::parsec::{VecState, State};

use std::iter::FromIterator;

#[test]
fn it_works() {
    let mut state:VecState<char> = VecState::from_iter("abc".chars().into_iter());
    assert_eq!(state.next(), Some('a'));
    assert_eq!(state.current(), 1);
    assert_eq!(state.next(), Some('b'));
    assert_eq!(state.current(), 2);
    assert_eq!(state.next(), Some('c'));
    assert_eq!(state.current(), 3);
    assert_eq!(state.next(), None);
    assert_eq!(state.current(), 3);
}
