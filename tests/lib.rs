#![feature(collections)]
#[macro_use]
extern crate ruskell;
use ruskell::parsec::{VecState, State, Status, Parsec, monad};
use ruskell::parsec::atom::{one, eof, one_of, none_of};
use ruskell::parsec::combinator::{either};//, bind, then, many, many1};
use std::sync::Arc;
use std::iter::FromIterator;

#[test]
fn state_works() {
    let mut state:VecState<char> = VecState::from_iter("abc".chars().into_iter());
    assert_eq!(state.next(), Some('a'));
    assert_eq!(state.pos(), 1);
    assert_eq!(state.next(), Some('b'));
    assert_eq!(state.pos(), 2);
    assert_eq!(state.next(), Some('c'));
    assert_eq!(state.pos(), 3);
    assert_eq!(state.next(), None);
    assert_eq!(state.pos(), 3);
}

#[test]
fn one_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one('a');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
    let a = one('b');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
    let a = one('c');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn one_eof_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = &mut one('a');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
    let a = &mut one('b');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
    let a = &mut one('c');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
    let re = eof()(&mut state);
    assert!(re.is_ok());
}

#[test]
fn one_of_test_0() {
    let es = "abc".chars().into_iter().collect::<Vec<char>>();
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let p = one_of(&es);
    let re = p(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn none_of_test_0() {
    let es = "abc".chars().into_iter().collect::<Vec<char>>();
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let p = none_of(&es);
    let re = p(&mut state);
    assert!(re.is_err());
}

#[test]
fn none_of_test_1() {
    let es = "bcdef".chars().into_iter().collect::<Vec<char>>();
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let p = none_of(&es);
    let re = p(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(one('a'));
    let b = Arc::new(one('b'));
    let e = &mut either(b, a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(one('a'));
    let b = Arc::new(one('b'));
    let e = &mut either(a, b);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(one('a'));
    let b = Arc::new(one('b'));
    let c = Arc::new(one('c'));
    let e = either(b, c).or(a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn monad_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one('a');
    let exp = monad(Arc::new(a)).bind(Arc::new(Box::new(move |state:&mut State<char>, x:char|->Status<Vec<char>>{
            one('b').parse(state).map(|y:char| -> Vec<char>{
                let mut res = Vec::new();
                res.push(x);
                res.push(y);
                res
            })
        }))).bind(
            Arc::new(Box::new(move |state: &mut State<char>, v:Vec<char>|->Status<Vec<char>>{
                one('c').parse(state).map(|x:char| -> Vec<char> {
                    let mut res = Vec::new();
                    res.push_all(&v);
                    res.push(x);
                    res
                })
        })));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b', 'c'];
    assert_eq!(data, ver);
}

#[test]
fn then_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one('a');
    let b = one('b');
    let c = one('c');
    let exp = monad(Arc::new(a)).over(Arc::new(b)).then(Arc::new(c));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}


#[test]
fn bind_then_over_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = one('a');
    let exp = monad(Arc::new(a)).then(Arc::new(one('b'))).over(Arc::new(one('c'))).over(Arc::new(eof()));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
}

// #[test]
// fn many_test_0() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('a'));
//     let re = many(a)(&mut state);
//     assert!(re.is_ok());
//     let data = re.unwrap();
//     let ver = Arc::new((vec!['a']).into_iter().map(|x:char| Arc::new(x)).collect());
//     assert_eq!(data, ver);
// }

// #[test]
// fn many_test_1() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('b'));
//     let re = many(a)(&mut state);
//     assert!(re.is_ok());
//     let data = re.unwrap();
//     let ver = Arc::new((vec![]).into_iter().map(|x:char| Arc::new(x)).collect());
//     assert_eq!(data, ver);
// }
//
// #[test]
// fn many_test_2() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('a'));
//     let b = one(Arc::new('b'));
//     let c = one(Arc::new('c'));
//
//     let re = many(either(a, b).or(c))(&mut state);
//     assert!(re.is_ok());
//     let data = re.unwrap();
//     let ver = Arc::new((vec!['a', 'b', 'c']).into_iter().map(|x:char| Arc::new(x)).collect());
//     assert_eq!(data, ver);
// }
//
// #[test]
// fn many1_test_0() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('a'));
//     let b = one(Arc::new('b'));
//     let c = one(Arc::new('c'));
//
//     let re = many1(either(a, b).or(c))(&mut state);
//     assert!(re.is_ok());
//     let data = re.unwrap();
//     let ver = Arc::new((vec!['a', 'b', 'c']).into_iter().map(|x:char| Arc::new(x)).collect());
//     assert_eq!(data, ver);
// }
//
// #[test]
// fn many1_test_1() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('b'));
//     let b = one(Arc::new('b'));
//     let c = one(Arc::new('c'));
//
//     let re = many1(either(a, b).or(c))(&mut state);
//     assert!(re.is_err());
// }
//
// #[test]
// fn many1_test_2() {
//     let mut state = VecState::from_iter("abc".chars().into_iter());
//     let a = one(Arc::new('a'));
//     let b = one(Arc::new('b'));
//
//     let re = many1(either(a, b))(&mut state);
//     assert!(re.is_ok());
//     let data = re.unwrap();
//     let ver = Arc::new((vec!['a', 'b']).into_iter().map(|x:char| Arc::new(x)).collect());
//     assert_eq!(data, ver);
// }
