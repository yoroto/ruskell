use std::vec::Vec;

pub trait Functor<A, B, F: FnMut(&A) -> B> {
    type Output;
    fn fmap(&self, f: F ) -> <Self as Functor<A, B, F>>::Output;
}

pub trait FunctorMut<A, B, F: FnMut(&A) -> B> {
    type Output;
    fn fmap(mut self, f: F) -> <Self as Functor<A, B, F>>::Output;
}

impl<A, B, F> Functor<A, B, F> for Vec<A>
where F:FnMut(&A)->B {
    type Output=Vec<B>;
    fn fmap(&self, f:F) -> Self::Output {
        self.into_iter().map(f).collect::<Self::Output>()
    }
}
