use crate::domain::Domain;
use std::{collections::HashMap, fmt::Display};

pub type Real = i32;

#[derive(Debug, Clone)]
pub struct Subst {
    map: HashMap<Term, Domain>,
}

impl Subst {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, t: &Term) -> Option<&Domain> {
        self.map.get(t)
    }

    pub fn is_bound(&self, t: &Term) -> bool {
        self.map.contains_key(t)
    }

    pub fn bind(&mut self, t: Term, value: Domain) -> bool {
        use std::collections::hash_map::Entry;
        match self.map.entry(t) {
            Entry::Vacant(e) => {
                e.insert(value);
                true
            }
            Entry::Occupied(_) => false,
        }
    }

    pub fn propagate(&mut self, from: &Term, to: &Term) -> bool {
        if let Some(v) = self.get(from) {
            return self.bind(to.clone(), v.clone());
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Term, &Domain)> {
        self.map.iter()
    }

    pub fn get_pair(&self, t1: &Term, t2: &Term) -> Option<(&Domain, &Domain)> {
        self.get(t1).zip(self.get(t2))
    }

    pub fn pair_owned(&self, t1: &Term, t2: &Term) -> Option<(Domain, Domain)> {
        self.get(t1).cloned().zip(self.get(t2).cloned())
    }

    pub fn refine(&mut self, t: Term, d: Domain) -> bool {
        use std::collections::hash_map::Entry;
        match self.map.entry(t) {
            Entry::Vacant(e) => {
                e.insert(d);
                true
            }
            Entry::Occupied(mut e) => {
                let new = e.get().intersect(&d);
                if &new != e.get() {
                    e.insert(new);
                    true
                } else {
                    false
                }
            }
        }
    }
}

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
