use std::fmt;
use std::str::Str;

// Lexicon for L0

#[deriving(Show)]
pub struct Noun<'dict>(pub &'dict str);

impl<'dict> Str for Noun<'dict> {
    fn as_slice(&self) -> &str {
        let &Noun(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Verb<'dict>(pub &'dict str);

impl<'dict> Str for Verb<'dict> {
    fn as_slice(&self) -> &str {
        let &Verb(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Adjective<'dict>(pub &'dict str);

impl<'dict> Str for Adjective<'dict> {
    fn as_slice(&self) -> &str {
        let &Adjective(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Pronoun<'dict>(pub &'dict str);

impl<'dict> Str for Pronoun<'dict> {
    fn as_slice(&self) -> &str {
        let &Pronoun(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct ProperNoun<'dict>(pub &'dict str);

impl<'dict> Str for ProperNoun<'dict> {
    fn as_slice(&self) -> &str {
        let &ProperNoun(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Determiner<'dict>(pub &'dict str);

impl<'dict> Str for Determiner<'dict> {
    fn as_slice(&self) -> &str {
        let &Determiner(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Preposition<'dict>(pub &'dict str);

impl<'dict> Str for Preposition<'dict> {
    fn as_slice(&self) -> &str {
        let &Preposition(word) = self;
        word
    }
}

#[deriving(Show)]
pub struct Conjunction<'dict>(pub &'dict str);

impl<'dict> Str for Conjunction<'dict> {
    fn as_slice(&self) -> &str {
        let &Conjunction(word) = self;
        word
    }
}

// Grammar for L0

#[deriving(Show)]
pub struct S<'dict, 'tree> {
    pub np: NP<'dict, 'tree>,
    pub vp: VP<'dict, 'tree>,
}

impl<'dict, 'tree> fmt::LowerExp for S<'dict, 'tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:e} {:e}", self.np, self.vp)
    }
}

#[deriving(Show)]
pub enum NP<'dict, 'tree> {
    ProNP(Pronoun<'dict>),
    PropNounNP(ProperNoun<'dict>),
    DetNomNP(Determiner<'dict>, Nominal<'dict, 'tree>),
}

impl<'dict, 'tree> fmt::LowerExp for NP<'dict, 'tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProNP(pro) => write!(f, "{:s}", pro),
            PropNounNP(pn) => write!(f, "{:s}", pn),
            DetNomNP(det ,nom) => write!(f, "{:s} {:e}", det, nom),
        }
    }
}

#[deriving(Show)]
pub struct Nominal<'dict, 'tree> {
    nouns: &'tree [Noun<'dict>],
}

impl<'dict, 'tree> Nominal<'dict, 'tree> {
    /// nouns must have length > 0
    pub fn make<'dict, 'tree>(nouns: &'tree [Noun<'dict>]) -> Nominal<'dict, 'tree> {
        assert!(nouns.len() > 0)
        Nominal { nouns: nouns }
    }

    pub fn nouns(&self) -> &[Noun] {
        self.nouns
    }
}

impl<'dict, 'tree> fmt::LowerExp for Nominal<'dict, 'tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.nouns.iter();
        // invariant: nouns have length > 0
        // Thus, it is safe to unwrap here
        try!(write!(f, "{:s}", *iter.next().unwrap()));
        for &noun in iter {
            try!(write!(f, " {:s}", noun))
        }
        Ok(())
    }
}

#[deriving(Show)]
pub struct VP<'dict, 'tree> {
    pub verb: Verb<'dict>,
    pub np: Option<NP<'dict, 'tree>>,
    pub pp: Option<PP<'dict, 'tree>>,
}

impl<'dict, 'tree> fmt::LowerExp for VP<'dict, 'tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:s}", self.verb));
        match self.np {
            Some(np) => try!(write!(f, " {:e}", np)),
            None => ()
        }
        match self.pp {
            Some(pp) => try!(write!(f, " {:e}", pp)),
            None => ()
        }
        Ok(())
    }
}

#[deriving(Show)]
pub struct PP<'dict, 'tree> {
    pub prep: Preposition<'dict>,
    pub np: NP<'dict, 'tree>,
}

impl<'dict, 'tree> fmt::LowerExp for PP<'dict, 'tree> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:s} {:e}", self.prep, self.np)
    }
}
