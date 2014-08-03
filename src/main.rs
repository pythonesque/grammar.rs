use grammar::l0::{Noun, Verb, Pronoun, Determiner, S, ProNP, DetNomNP, Nominal, VP};

mod grammar;

fn main() {
    let nouns = [Noun("morning"), Noun("flight")];
    let s = S {
        np: ProNP(Pronoun("I")),
        vp: VP {
            verb: Verb("prefer"),
            np: Some(DetNomNP(Determiner("a"), Nominal::make(nouns))),
            pp: None,
        },
    };
    println!("{:e}", s);
    println!("{}", s);
}
