#[macro_use]
extern crate ruskell;
use std::fmt;
use std::vec;
use ruskell::functional::fixed::{y, RecFunc};
use std::sync::Arc;
use std::boxed::Box;

#[test]
fn fib_test() {
    let fib : RecFunc<i32, i32> = abc!(|f| abc!(move |x| if (x<2) { 1 } else { f(x-1) + f(x-2)}));
    let b = y(fib)(10);
    assert_eq!(b, 89);
}

#[test]
fn fac_test() {
    let fac : RecFunc<i32, i32> = abc!(|f| abc!(move |x| if (x==0) { 1 } else { f(x-1) * x }));
    let c = y(fac)(10);
    assert_eq!(c, 3628800);
}
