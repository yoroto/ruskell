use parsec::{State, SimpleError, Status, Parsec, Monad, M, Bind, parser, bind};
use parsec::combinator::{Either, either, try, many1};
use parsec::atom::{OneOf, pack, eq, one_of};
use std::sync::Arc;
use std::boxed::Box;

pub fn space() -> OneOf<char> {
    one_of(&vec![' ', '\t'])
}

pub fn white_space() -> Bind<char, char> {
    bind(bnd!(|state: &mut State<char>, x:char|->Status<char> {
        if x.is_whitespace() {
            Ok(x)
        } else {
            let message = format!("Expect space but got {}", x);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }))
}

pub fn newline() -> Either<char, String> {
    let rel = eq('\r');
    let nl = eq('\n');
    let thn = arc!(either(arc!(try(arc!(nl.clone())).then(arc!(pack(String::from("\r\n"))))),
                            arc!(pack(String::from("\r")))));
    either(arc!(rel.then(thn.clone())), arc!(nl.then(arc!(pack(String::from("\n"))))))
}

pub fn digit() -> Bind<char, char> {
    bind(bnd!(|state: &mut State<char>, x:char|->Status<char> {
        if x.is_numeric() {
            Ok(x)
        } else {
            let message = format!("Expect numeric but got {}", x);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }))
}

pub fn alpha() -> Bind<char, char> {
    bind(bnd!(|state: &mut State<char>, x:char|->Status<char> {
        if x.is_alphabetic() {
            Ok(x)
        } else {
            let message = format!("Expect alphabetic but got {}", x);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }))
}

pub fn alphanumeric() -> Bind<char, char> {
    bind(bnd!(|state: &mut State<char>, x:char|->Status<char> {
        if x.is_alphanumeric() {
            Ok(x)
        } else {
            let message = format!("Expect alphabetic or number but got {}", x);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }))
}

pub fn control() -> Bind<char, char> {
    bind(bnd!(|state: &mut State<char>, x:char|->Status<char> {
        if x.is_control() {
            Ok(x)
        } else {
            let message = format!("Expect control but got {}", x);
            Err(SimpleError::new(state.pos(), String::from(message)))
        }
    }))
}

pub fn uinteger() -> Monad<char, Vec<char>, String> {
    parser(arc!(many1(arc!(digit())))).bind(bnd!(|_:&mut State<char>, x:Vec<char>| -> Status<String> {
        Ok(x.iter().cloned().collect::<String>())
    }))
}

pub fn integer() ->Either<char, String>{
    either(arc!(try(arc!(eq('-'))).bind(bnd!(|state: &mut State<char>, _:char|-> Status<String> {
        uinteger().parse(state).map(|x:String|->String{
            let mut re = String::from("-");
            re.push_str(x.as_str());
            re
        })
    }))), arc!(uinteger()))
}

pub fn ufloat() -> Monad<char, String, String> {
    let left = either(arc!(uinteger()), arc!(pack(String::from("0"))));
    let right = uinteger();
    left.over(arc!(eq('.'))).bind(bnd!(move |state: &mut State<char>, x:String|->Status<String> {
        let right = right.clone();
        let rer = right.parse(state);
        rer.map(|r:String|->String{
            let mut re = String::from(x.as_str());
            re.push('.');
            re.push_str(r.as_str());
            re
        })
    }))
}

pub fn float() ->Either<char, String>{
    either(arc!(try(arc!(eq('-'))).bind(bnd!(|state: &mut State<char>, _:char|-> Status<String> {
        ufloat().parse(state).map(|x:String|->String{
            let mut re = String::from("-");
            re.push_str(x.as_str());
            re
        })
    }))), arc!(ufloat()))
}
