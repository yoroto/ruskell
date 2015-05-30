#![feature(box_syntax, box_patterns)]
use functional::functor::Functor;
use std::result::Result;
use std::option::Option;

pub trait Sequential<Ax, Bx, Fx, Ay, By, Y:Functor<Ay, By, Box<Ax>>> : Functor<Box<Ax>, Bx, Fx> {
    type Seq;
    fn seq(&self, &Y)->Self::Seq;
}

impl<Ax, Fx, Ay, By, Y> Sequential<Ax, Y::Output, Fx, Ay, By, Y> for Vec<Box<Ax>>
where Y:Functor<Ay, By, Box<Ax>>, Fx:Fn(&Box<Ax>)->Y::Output, Ax:Fn(&Ay)->By {
    type Seq = Vec<Y::Output>;
    fn seq(&self, y:&Y)->Self::Seq {
        self.fmap(&|f:&Box<Ax>| y.fmap(f))
    }
}
//
// impl<Ax:Copy, E:Copy, Fx, Ay:Copy, By:Copy, Y:Functor<Ay, By, Ax>> Sequential<Ax, Y::Output, Fx, Ay, By, Y>
//     for Result<Ax, E>
// where Fx: Fn(Ax)->Y::Output, Y::Output:Copy {
//     type Seq = Result<Y::Output, E>;
//     fn seq(&self, y:&Y)-><Self as Sequential<Ax, Y::Output, Fx, Ay, By, Y>>::Seq {
//         self.fmap(&|f:Ax| y.fmap(&f))
//     }
// }
//
// impl<Ax:Copy, Fx, Ay:Copy, By:Copy, Y:Functor<Ay, By, Ax>> Sequential<Ax, Y::Output, Fx, Ay, By, Y>
//     for Option<Ax>
// where Fx: Fn(Ax)->Y::Output, Y::Output:Copy {
//     type Seq = Option<Y::Output>;
//     fn seq(&self, y:&Y)-><Self as Sequential<Ax, Y::Output, Fx, Ay, By, Y>>::Seq {
//         self.fmap(&|f:Ax| y.fmap(&f))
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use functional::applicative::Sequential;
//     use functional::functor::Functor;
//     #[test]
//     fn vec_x_vec() {
//         let left:Vec<Box<Fn(&i32)->i32>> = vec![
//                 Box::new(|x:&i32|*x),
//             ];
//
//         let right:Vec<i32> = vec![0, 1, 2, 3, 4];
//         let data:Vec<Vec<i32>> = left.seq(&right);
//         let verify = vec![vec![0, 1, 2, 3, 4]];
//         for i in 0..verify.len() {
//             assert_eq!(verify[i], data[i]);
//         }
//     }
// }
