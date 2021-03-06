use parsec::{State, Parsec, Status, Monad, monad, M, parser};
use parsec::atom::{pack, fail};
use std::sync::Arc;
use std::fmt::{Debug, Formatter};
use std::fmt;

pub struct Try<T, R>{
    parsec : Arc<Parsec<T, R>>,
}

impl<T, R> Try<T, R> where T:Clone {
    pub fn new(p:Arc<Parsec<T, R>>) -> Try<T, R> {
        Try{parsec:p.clone()}
    }
}

impl<T, R> Parsec<T, R> for Try<T, R> where T:Clone {
    fn parse(&self, state: &mut State<T>)->Status<R> {
        let pos = state.pos();
        let res = self.parsec.parse(state);
        if res.is_err() {
            state.seek_to(pos);
        }
        res
    }
}

impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Try<T, R> where T:Clone {
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Try<T, R> where T:Clone {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> Fn<(&'a mut State<T>, )> for Try<T, R> where T:Clone {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Try<T, R> where T:Clone {
    fn clone(&self)->Self {
        Try{parsec:self.parsec.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
    }
}

impl<T, R> Debug for Try<T, R> where T:Clone{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<try parsec>".fmt(formatter)
    }
}

impl<T:'static+Clone, R:'static+Clone> M<T, R> for Try<T, R>{}

pub fn try<T, R>(p:Arc<Parsec<T, R>>) -> Try<T, R> where T:Clone {
    Try::new(p)
}

pub struct Either<T, R>{
    x: Arc<Parsec<T, R>>,
    y: Arc<Parsec<T, R>>,
}

impl<T:'static, R:'static> Either<T, R> where T:Clone{
    pub fn new(x:Arc<Parsec<T, R>>, y:Arc<Parsec<T, R>>) -> Either<T, R> {
        Either{x:x.clone(), y:y.clone()}
    }

    pub fn or(&self, z:Arc<Parsec<T, R>>)-> Either<T, R> {
        let left = Either{x:self.x.clone(), y:self.y.clone()};
        Either::new(Arc::new(left), z.clone())
    }
}

impl<T, R> Parsec<T, R> for Either<T, R> where T:Clone{
    fn parse(&self, state:&mut State<T>)->Status<R> {
        let pos = state.pos();
        let val = self.x.parse(state);
        if val.is_ok() {
            val
        } else {
            if pos == state.pos() {
                self.y.parse(state)
            } else {
                val
            }
        }
    }
}

impl<'a, T, R> FnOnce<(&'a mut State<T>, )> for Either<T, R> where T:Clone{
    type Output = Status<R>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> FnMut<(&'a mut State<T>, )> for Either<T, R> where T:Clone{
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<R> {
        panic!("Not implement!");
    }
}

impl<'a, T, R> Fn<(&'a mut State<T>, )> for Either<T, R> where T:Clone{
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<R> {
        //self.call_once(args)
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Either<T, R> where T:Clone {
    fn clone(&self)->Self {
        Either{x:self.x.clone(), y:self.y.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.x = source.x.clone();
        self.y = source.y.clone();
    }
}

impl<T, R> Debug for Either<T, R> where T:Clone {
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<either parsec>".fmt(formatter)
    }
}

impl<T:'static+Clone, R:'static+Clone> M<T, R> for Either<T, R>{}

pub fn either<T:'static, R:'static>(x: Arc<Parsec<T, R>>, y:Arc<Parsec<T, R>>)->Either<T, R> where T:Clone{
    Either::new(x, y)
}

pub fn many<T:'static, R:'static>(p:Arc<Parsec<T, R>>)->Either<T, Vec<R>>
where T:Clone, R:Clone+Debug {
    either(Arc::new(many1(Arc::new(try(p)))), Arc::new(pack(Vec::new())))
}

pub fn many1<T:'static, R:'static>(p:Arc<Parsec<T, R>>)->Monad<T, R, Vec<R>> where T:Clone, R:Clone+Debug {
    parser(p.clone()).bind(Arc::new(Box::new(move |state: &mut State<T>, x: R| -> Status<Vec<R>> {
        let mut rev = Vec::new();
        let tail = many(p.clone()).parse(state);
        let data = tail.unwrap();
        rev.push(x);
        rev.push_all(&data);
        Ok(rev)
    })))
}

pub fn between<T:'static, B:'static, P:'static, E:'static>
        (begin:Arc<Parsec<T, B>>, parsec:Arc<Parsec<T, P>>, end:Arc<Parsec<T, E>>)
        ->Monad<T, P, P> where T:Clone, P:Clone, B:Clone, E:Clone {
    // TODO: A fake binder between begin and parsec then, someone manybe remove it.
    parser(begin).then(parsec).over(end)
}

pub fn otherwise<T:'static, R:'static>(p:Arc<Parsec<T, R>>, message:String)->Either<T, R>
where T:Clone, R:Clone {
    either(p.clone(), Arc::new(fail(message)))
}

pub fn many_tail<T:'static, R:'static, Tail:'static>(p:Arc<Parsec<T, R>>, tail:Arc<Parsec<T, Tail>>)
    ->Monad<T, Vec<R>, Vec<R>>
where T:Clone, R:Clone+Debug, Tail:Clone{
    // TODO: A fake binder between p and tail, someone manybe remove it.
    parser(Arc::new(many(p))).over(tail)
}

pub fn many1_tail<T:'static, R:'static, Tail:'static>(p:Arc<Parsec<T, R>>, tail:Arc<Parsec<T, Tail>>)
    ->Monad<T, Vec<R>, Vec<R>>
where T:Clone, R:Clone+Debug, Tail:Clone{
    // TODO: A fake binder between p and tail, someone manybe remove it.
    parser(Arc::new(many1(p))).over(tail)
}

// We can use many/many1 as skip, but them more effective.
pub struct Skip<T, R> {
    parsec: Arc<Parsec<T, R>>,
}

impl<T, R> Skip<T, R> where T:Clone, R:Clone+Debug {
    pub fn new(p:Arc<Parsec<T, R>>) -> Skip<T, R> {
        Skip{parsec:p.clone()}
    }
}

impl<T:'static, R:'static> Parsec<T, Vec<R>> for Skip<T, R> where T:Clone, R:Clone+Debug {
    fn parse(&self, state:&mut State<T>)->Status<Vec<R>> {
        loop {
            let re = try(self.parsec.clone()).parse(state);
            if re.is_err() {
                return Ok(Vec::new())
            }
        }
    }
}

impl<'a, T:'static, R:'static> FnOnce<(&'a mut State<T>, )> for Skip<T, R> where T:Clone, R:Clone+Debug{
    type Output = Status<Vec<R>>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<Vec<R>> {
        panic!("Not implement!");
    }
}

impl<'a, T:'static, R:'static> FnMut<(&'a mut State<T>, )> for Skip<T, R> where T:Clone, R:Clone+Debug{
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<Vec<R>> {
        panic!("Not implement!");
    }
}

impl<'a, T:'static, R:'static> Fn<(&'a mut State<T>, )> for Skip<T, R> where T:Clone, R:Clone+Debug{
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<Vec<R>> {
        //self.call_once(args)
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T, R> Clone for Skip<T, R> where T:Clone, R:Clone+Debug {
    fn clone(&self)->Self {
        Skip{parsec:self.parsec.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
    }
}

impl<T, R> Debug for Skip<T, R> where T:Clone, R:Clone+Debug{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<skip parsec>".fmt(formatter)
    }
}

impl<T:'static+Clone, R:'static+Clone+Debug> M<T, Vec<R>> for Skip<T, R>{}

pub fn skip_many<T:'static, R:'static>(p:Arc<Parsec<T, R>>)->Skip<T, R> where T:Clone, R:Clone+Debug {
    Skip::new(p)
}

pub struct Skip1<T, R> {
    parsec: Arc<Parsec<T, R>>,
}

impl<T, R> Skip1<T, R> where T:Clone, R:Clone+Debug {
    pub fn new(p:Arc<Parsec<T, R>>) -> Skip1<T, R> {
        Skip1{parsec:p.clone()}
    }
}

impl<T:'static, R:'static> Parsec<T, Vec<R>> for Skip1<T, R> where T:Clone, R:Clone+Debug {
    fn parse(&self, state:&mut State<T>)->Status<Vec<R>> {
        let re = self.parsec.parse(state);
        if re.is_err() {
            Err(re.unwrap_err())
        } else {
            skip_many(self.parsec.clone()).parse(state)
        }
    }
}

impl<T, R> Clone for Skip1<T, R> where T:Clone, R:Clone+Debug {
    fn clone(&self)->Self {
        Skip1{parsec:self.parsec.clone()}
    }

    fn clone_from(&mut self, source: &Self) {
        self.parsec = source.parsec.clone();
    }
}

impl<T, R> Debug for Skip1<T, R> where T:Clone, R:Clone+Debug{
    fn fmt(&self, formatter:&mut Formatter)->Result<(), fmt::Error> {
        "<many1 parsec>".fmt(formatter)
    }
}

impl<'a, T:'static, R:'static> FnOnce<(&'a mut State<T>, )> for Skip1<T, R> where T:Clone, R:Clone+Debug{
    type Output = Status<Vec<R>>;
    extern "rust-call" fn call_once(self, _: (&'a mut State<T>, )) -> Status<Vec<R>> {
        panic!("Not implement!");
    }
}

impl<'a, T:'static, R:'static> FnMut<(&'a mut State<T>, )> for Skip1<T, R> where T:Clone, R:Clone+Debug {
    extern "rust-call" fn call_mut(&mut self, _: (&'a mut State<T>, )) -> Status<Vec<R>> {
        panic!("Not implement!");
    }
}

impl<'a, T:'static, R:'static> Fn<(&'a mut State<T>, )> for Skip1<T, R> where T:Clone, R:Clone+Debug {
    extern "rust-call" fn call(&self, args: (&'a mut State<T>, )) -> Status<Vec<R>> {
        //self.call_once(args)
        let (state, ) = args;
        self.parse(state)
    }
}

impl<T:'static+Clone, R:'static+Clone+Debug> M<T, Vec<R>> for Skip1<T, R>{}

pub fn skip_many1<T:'static, R:'static>(p:Arc<Parsec<T, R>>)->Skip1<T, R> where T:Clone, R:Clone+Debug {
    Skip1::new(p)
}

pub fn sep_by<T:'static, Sep:'static, R:'static>(sep:Arc<Parsec<T, Sep>>, parsec:Arc<Parsec<T, R>>)->Either<T, Vec<R>>
where T:Clone, R:Clone+Debug, Sep:Clone{
    let s = Arc::new(try(sep));
    let p = Arc::new(try(parsec));
    either(Arc::new(sep_by1(s, p)), Arc::new(pack(Vec::new())))
}

pub fn sep_by1<T:'static, Sep:'static, R:'static>(sep:Arc<Parsec<T, Sep>>, parsec:Arc<Parsec<T, R>>)
    ->Monad<T, R, Vec<R>>
where T:Clone, R:Clone+Debug, Sep:Clone{
    monad(parsec.clone()).bind(Arc::new(Box::new(move |state:&mut State<T>, x:R|->Status<Vec<R>>{
        let mut rev = Vec::new();
        let tail = sep_by(sep.clone(), parsec.clone()).parse(state);
        let data = tail.unwrap();
        rev.push(x);
        rev.push_all(&data);
        Ok(rev)
    })))
}
