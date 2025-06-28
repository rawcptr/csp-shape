use std::collections::HashMap;

pub type Real = i32;
pub type Subst = HashMap<Term, Real>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Term {
    Var(&'static str), // something like "t1" or "t2" or something.
    Val(Real),         // stand in for our Shape
}

impl From<&'static str> for Term {
    fn from(value: &'static str) -> Self {
        Term::Var(value)
    }
}

impl From<Real> for Term {
    fn from(value: Real) -> Self {
        Term::Val(value)
    }
}
