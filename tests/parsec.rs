#![feature(vec_push_all)]
#[macro_use]
extern crate ruskell;
use ruskell::parsec::{VecState, State, Status, Parsec, Monad};
use ruskell::parsec::atom::{one, eq, eof, one_of, none_of, ne};
use ruskell::parsec::combinator::{try, either, many, many1, between, many_tail, many1_tail, Either, Or};
use std::sync::Arc;
use std::iter::FromIterator;
use std::error::Error;

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
    let a = try(eq('a'));
    let b = try(eq('b'));
    let e = either(b, a);
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let e = either(try(a), try(b));
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn either_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');
    let e:Either<char, char> = either(try(b), try(c)).or(try(a));
    let re = e(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'a');
}

#[test]
fn monad_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = a.bind(abc!(move |state:&mut State<char>, x:char|->Status<Vec<char>>{
            eq('b').parse(state).map(|y:char| -> Vec<char>{
                let mut res = Vec::new();
                res.push(x);
                res.push(y);
                res
            })
        })).bind(abc!(move |state: &mut State<char>, v:Vec<char>|->Status<Vec<char>>{
                eq('c').parse(state).map(|x:char| -> Vec<char> {
                    let mut res = Vec::new();
                    res.push_all(&v);
                    res.push(x);
                    res
                })
        }));
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
    let exp = a.over(b).then(c);
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn bind_then_over_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');
    let re = a.then(b).over(c).parse(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
}

#[test]
fn parser_then_over_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = a.then(eq('b')).over(eq('c')).over(eof());
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
    let exp = a.over(b).then(c).over(eof());
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'c');
}

#[test]
fn m_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let exp = a.then(eq('b')).over(eq('c')).over(eof());
    let re = exp(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    assert_eq!(data, 'b');
}

#[test]
fn many_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let re = many(a)(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a'];
    assert_eq!(data, ver);
}

#[test]
fn many_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('b');
    let re = many(a).parse(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec![];
    assert_eq!(data, ver);
}

#[test]
fn many_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');

    let re = many(either(try(a), try(b)).or(try(c)))(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b', 'c'];
    assert_eq!(data, ver);
}

#[test]
fn many1_test_0() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');
    let c = eq('c');

    let re = many1(either(try(a), try(b)).or(try(c)))(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b', 'c'];
    assert_eq!(data, ver);
}

#[test]
fn many1_test_1() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('b');
    let b = eq('b');
    let c = eq('c');

    let re = many1(either(try(a), try(b)).or(try(c)))(&mut state);
    assert!(re.is_err());
}

#[test]
fn many1_test_2() {
    let mut state = VecState::from_iter("abc".chars().into_iter());
    let a = eq('a');
    let b = eq('b');

    let re = many1(either(try(a), try(b)))(&mut state);
    assert!(re.is_ok());
    let data = re.unwrap();
    let ver = vec!['a', 'b'];
    assert_eq!(data, ver);
}

#[test]
fn between_test_0() {
    let mut state = VecState::from_iter("\"xxxxxxxx\".".chars());
    let quote = eq('\"');

    let content = many(eq('x'));
    let re = between(quote.clone(), quote.clone(), content)(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "xxxxxxxx".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}

#[test]
fn between_test_1() {
    let mut state = VecState::from_iter("This is a string in quotes: \"xxxxxxxx\".".chars());
    let prefix = many(ne('\"'));
    let quote = eq('\"');
    let content = many(eq('x'));
    let re = prefix.then(between(quote.clone(), quote.clone(), content))(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "xxxxxxxx".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}

#[test]
fn many_tail_test_0() {
    let mut state = VecState::from_iter("This is a string.".chars());
    let content = many_tail(ne('.'), eq('.'));
    let re = content(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "This is a string".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}

#[test]
fn many_tail_test_1() {
    let mut state = VecState::from_iter("This is a string.".chars());
    let content = many_tail(one(), eof());
    let re = content(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "This is a string.".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}

#[test]
fn many1_tail_test_0() {
    let mut state = VecState::from_iter("This is a string.".chars());
    let content = many1_tail(ne('.'), eq('.'));
    let re = content(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "This is a string".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}

#[test]
fn many1_tail_test_1() {
    let mut state = VecState::from_iter("This is a string.".chars());
    let content = many1_tail(one(), eof());
    let re = content(&mut state);
    if re.is_err() {
        let msg = format!("{}", re.unwrap_err().description());
        panic!(msg);
    }
    let data = re.unwrap();
    let ver = "This is a string.".chars().into_iter().collect::<Vec<char>>();
    assert_eq!(data, ver);
}
