use parsec::{State, Status, Parsec, Monad, Parser, parser};
use parsec::combinator::{either, try, many1};
use parsec::atom::{OneOf, pack, eq, one_of};
use std::sync::Arc;
use std::boxed::Box;

pub fn space() -> OneOf<char> {
    one_of(&vec![' ', '\t'])
}

pub fn white_space() -> Parser<char, char> {
    parser(abc!(|state: &mut State<char>| -> Status<char>{
        state.next_by(&|x:&char| x.is_whitespace())
    }))
}

pub fn newline() -> Parser<char, String> {
    parser(abc!(|state: &mut State<char>| -> Status<String>{
        let rel = eq('\r');
        let nl = eq('\n');
        let thn = either(try(nl.clone()).then(pack(String::from("\r\n"))),
                                pack(String::from("\r")));
        either(rel.then(thn.clone()), nl.then(pack(String::from("\n")))).parse(state)
    }))
}

pub fn digit() -> Parser<char, char> {
    parser(abc!(|state: &mut State<char>| -> Status<char>{
        state.next_by(&|x:&char| x.is_numeric())
    }))
}

pub fn alpha() -> Parser<char, char> {
    parser(abc!(|state: &mut State<char>| -> Status<char>{
        state.next_by(&|x:&char| x.is_alphabetic())
    }))
}

pub fn alphanumeric() -> Parser<char, char> {
    parser(abc!(|state: &mut State<char>| -> Status<char>{
        state.next_by(&|x:&char| x.is_alphanumeric())
    }))
}

pub fn control() -> Parser<char, char> {
    parser(abc!(|state: &mut State<char>| -> Status<char>{
        state.next_by(&|x:&char| x.is_control())
    }))
}

pub fn uinteger() -> Parser<char, String> {
    parser(abc!(|state: &mut State<char>|-> Status<String> {
        many1(digit()).bind(abc!(|_:&mut State<char>, x:Vec<char>| -> Status<String> {
            Ok(x.iter().cloned().collect::<String>())
        })).parse(state)
    }))
}

pub fn integer() ->Parser<char, String>{
    parser(abc!(|state: &mut State<char>|->Status<String>{
        either(try(eq('-')).bind(abc!(|state: &mut State<char>, _:char|-> Status<String> {
            uinteger().parse(state).map(|x:String|->String{
                let mut re = String::from("-");
                re.push_str(x.as_str());
                re
            })
        })), uinteger()).parse(state)
    }))
}

pub fn ufloat() -> Parser<char, String> {
    parser(abc!(|state: &mut State<char>|->Status<String>{
        let left = either(uinteger(), pack(String::from("0")));
        let right = uinteger();
        left.over(eq('.')).bind(abc!(move |state: &mut State<char>, x:String|->Status<String> {
            let right = right.clone();
            let rer = right.parse(state);
            rer.map(|r:String|->String{
                let mut re = String::from(x.as_str());
                re.push('.');
                re.push_str(r.as_str());
                re
            })
        })).parse(state)
    }))
}

pub fn float() -> Parser<char, String>{
    parser(abc!(|state:&mut State<char>|->Status<String>{
        either(try(eq('-')).bind(abc!(|state: &mut State<char>, _:char|-> Status<String> {
            ufloat().parse(state).map(|x:String|->String{
                let mut re = String::from("-");
                re.push_str(x.as_str());
                re
            })
        })), ufloat()).parse(state)
    }))
}
