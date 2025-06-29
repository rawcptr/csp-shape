use std::{collections::HashMap, fmt::Display};

pub type Real = i32;
pub type Subst = HashMap<Term, Real>;

#[derive(Hash, PartialEq, Eq, Clone)]
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

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{v}",),
            Term::Val(r) => write!(f, "{r}"),
        }
    }
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "Variable({})", v),
            Term::Val(r) => write!(f, "Real({})", r),
        }
    }
}

