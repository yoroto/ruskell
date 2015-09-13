#![feature(core)]
#![feature(convert)]
#![feature(unboxed_closures)]
#![feature(vec_push_all)]
#![feature(custom_derive)]

//Arc<Box<Closure>>
#[macro_export]
macro_rules! abc {
    ($x:expr) => (Arc::new(Box::new($x)))
}

pub mod functional;
pub mod parsec;
