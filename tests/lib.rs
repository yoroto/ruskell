#![feature(vec_push_all)]
#[macro_use]
extern crate ruskell;
use ruskell::parsec::{VecState, State, Status, Parsec, Error, monad, M};
use ruskell::parsec::atom::{eq, eof, one_of, none_of};
use ruskell::parsec::combinator::{either, many, many1, between};
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
fn eq_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
    let a = eq('b');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
    let a = eq('c');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn eq_eof_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = &mut eq('a');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
    let a = &mut eq('b');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
    let a = &mut eq('c');
    let re = a(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
    let re = eof()(&mut state);
    assert!(re.is_ok());
}

#[test]
fn eq_of_test_0() {
    let es = "abc".chars().into_iter().collect::<Vec<char>>();
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let p = one_of(&es);
    let re = p(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn neq_of_test_0() {
    let es = "abc".chars().into_iter().collect::<Vec<char>>();
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let p = none_of(&es);
    let re = p(&mut state);
    assert!(re.is_err());
}

#[test]
fn neq_of_test_1() {
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
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));
    let e = &mut either(b, a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));
    let e = &mut either(a, b);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));
    let c = Arc::new(eq('c'));
    let e = either(b, c).or(a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn monad_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = monad(Arc::new(a)).bind(Arc::new(Box::new(move |state:&mut State<char>, x:char|->Status<Vec<char>>{
            eq('b').parse(state).map(|y:char| -> Vec<char>{
                let mut res = Vec::new();
                res.push(x);
                res.push(y);
                res
            })
        }))).bind(
            Arc::new(Box::new(move |state: &mut State<char>, v:Vec<char>|->Status<Vec<char>>{
                eq('c').parse(state).map(|x:char| -> Vec<char> {
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
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');
    let exp = monad(Arc::new(a)).over(Arc::new(b)).then(Arc::new(c));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn bind_then_over_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = monad(Arc::new(a)).then(Arc::new(eq('b'))).over(Arc::new(eq('c'))).over(Arc::new(eof()));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
}

#[test]
fn m_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');
    let exp = a.over(Arc::new(b)).then(Arc::new(c));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn m_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = a.then(Arc::new(eq('b'))).over(Arc::new(eq('c'))).over(Arc::new(eof()));
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
}

#[test]
fn many_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let re = many(a).parse(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a'];
    assert_eq!(data, ver);
}

#[test]
fn many_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('b'));
    let re = many(a).parse(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec![];
    assert_eq!(data, ver);
}

#[test]
fn many_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));
    let c = Arc::new(eq('c'));

    let re = many(Arc::new(either(a, b).or(c))).parse(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b', 'c'];
    assert_eq!(data, ver);
}

#[test]
fn many1_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));
    let c = Arc::new(eq('c'));

    let re = many1(Arc::new(either(a, b).or(c)))(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b', 'c'];
    assert_eq!(data, ver);
}

#[test]
fn many1_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('b'));
    let b = Arc::new(eq('b'));
    let c = Arc::new(eq('c'));

    let re = many1(Arc::new(either(a, b).or(c)))(&mut state);
    assert!(re.is_err());
}

#[test]
fn many1_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = Arc::new(eq('a'));
    let b = Arc::new(eq('b'));

    let re = many1(Arc::new(either(a, b)))(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b'];
    assert_eq!(data, ver);
}

#[test]
fn between_test_0() {
    let mut state = VecState::from_iter("\"xxxxxxxx\".".chars());
    let quote = Arc::new(eq('\"'));

    let content = Arc::new(many(Arc::new(eq('x'))));
    let re = between(quote.clone(), content, quote)(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().message());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "xxxxxxxx".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}
