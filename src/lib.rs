use std::collections::hash_map::Entry;

use crate::{
    constraint::Constraint,
    term::{Real, Subst, Term},
};

pub mod constraint;
pub mod term;

// Term1 == Term2
#[derive(Debug)]
pub struct EqualityConstraint {
    t1: Term,
    t2: Term,
}

impl EqualityConstraint {
    pub fn boxed(t1: Term, t2: Term) -> Box<Self> {
        Box::new(Self { t1, t2 })
    }
}

impl Constraint for EqualityConstraint {
    fn hold(&self, subst: &Subst) -> bool {
        match (subst.get(&self.t1), subst.get(&self.t2)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    fn make_progress(&self, subst: &mut Subst) -> bool {
        // println!("t1: {:?}, t2: {:?}", &self.0, &self.1);
        match (&self.t1, &self.t2) {
            // X = Y case - need to check if either is already assigned
            (Term::Var(x), Term::Var(y)) => {
                if let Some(&v) = subst.get(&Term::Var(x)) {
                    if let Entry::Vacant(e) = subst.entry(Term::Var(y)) {
                        e.insert(v);
                        let s = format!("made progress: {:?} = {:?}", &self.t1, &self.t2);
                        dbg!(s);
                        return true;
                    }
                }

                // Try the other direction too
                if let Some(&v) = subst.get(&Term::Var(y)) {
                    if let Entry::Vacant(e) = subst.entry(Term::Var(x)) {
                        e.insert(v);
                        let s = format!("made progress: {:?} = {:?}", &self.t1, &self.t2);
                        dbg!(s);
                        return true;
                    }
                }
                false // no progress made
            }

            // X = 5 case - just assign X = 5
            (Term::Var(x), Term::Val(v)) | (Term::Val(v), Term::Var(x)) => {
                if let Entry::Vacant(e) = subst.entry(Term::Var(x)) {
                    e.insert(*v);
                    let s = format!("made progress: {:?} = {:?}", &self.t1, &self.t2);
                    dbg!(s);
                    return true;
                }
                false
            }
            // 5 = 5 case - nothing to do
            (_, _) => false,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LessThanConstraint {
    t1: Term,
    t2: Term,
}

impl LessThanConstraint {
    pub fn boxed(t1: Term, t2: Term) -> Box<Self> {
        Box::new(Self { t1, t2 })
    }
}

impl Constraint for LessThanConstraint {
    fn hold(&self, subst: &Subst) -> bool {
        match (subst.get(&self.t1), subst.get(&self.t2)) {
            (Some(a), Some(b)) => a < b,
            _ => true,
        }
    }

    fn make_progress(&self, _subst: &mut Subst) -> bool {
        false
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GreaterThanConstraint {
    t1: Term,
    t2: Term,
}

impl GreaterThanConstraint {
    pub fn boxed(t1: Term, t2: Term) -> Box<Self> {
        Box::new(Self { t1, t2 })
    }
}

impl Constraint for GreaterThanConstraint {
    fn hold(&self, subst: &Subst) -> bool {
        match (subst.get(&self.t1), subst.get(&self.t2)) {
            (Some(a), Some(b)) => a > b,
            _ => true,
        }
    }

    fn make_progress(&self, _subst: &mut Subst) -> bool {
        false
    }
}

pub struct BinaryOpConstraint {
    op: &'static str, // for debugging
    a: Term,
    b: Term,
    c: Term,

    e1: fn(Real, Real) -> Real, // a op b = c
    e2: fn(Real, Real) -> Real, // c op a = b
    e3: fn(Real, Real) -> Real, // b op a = c
}

impl BinaryOpConstraint {
    pub fn sum(a: Term, b: Term, c: Term) -> Box<Self> {
        Box::new(Self {
            a,
            b,
            c,
            e1: |x, y| x + y, // a + b = c
            e2: |y, z| z - y, // c - b = a
            e3: |x, z| z - x, // c - a = b
            op: "sum",
        })
    }

    pub fn name(&self) -> &str {
        self.op
    }

    pub fn product(a: Term, b: Term, c: Term) -> Box<Self> {
        Box::new(Self {
            a,
            b,
            c,
            e1: |x, y| x * y, // a * b = c
            e2: |y, z| z / y, // c / b = a
            e3: |x, z| z / x, // c / a = b
            op: "product",
        })
    }
}

impl Constraint for BinaryOpConstraint {
    fn hold(&self, subst: &Subst) -> bool {
        match (subst.get(&self.a), subst.get(&self.b), subst.get(&self.c)) {
            (Some(a), Some(b), Some(c)) => (self.e1)(*a, *b) == *c,
            _ => true,
        }
    }

    fn make_progress(&self, subst: &mut Subst) -> bool {
        // if a, b known, find c
        // if a, c known, find b
        // if b, c known, find a
        if let (Some(&a), Some(&b)) = (subst.get(&self.a), subst.get(&self.b)) {
            if let Entry::Vacant(e) = subst.entry(self.c.clone()) {
                e.insert((self.e1)(a, b));
                let s = format!(
                    "made progress: {:?} {} {:?} = {:?}",
                    &self.a, &self.op, &self.b, &self.c
                );
                dbg!(s);
                return true;
            }
        }
        if let (Some(&c), Some(&b)) = (subst.get(&self.c), subst.get(&self.b)) {
            if let Entry::Vacant(e) = subst.entry(self.a.clone()) {
                e.insert((self.e2)(c, b));
                let s = format!(
                    "made progress: {:?} {} {:?} = {:?}",
                    &self.c, &self.op, &self.b, &self.a
                );
                dbg!(s);
                return true;
            }
        }
        if let (Some(&c), Some(&a)) = (subst.get(&self.c), subst.get(&self.a)) {
            if let Entry::Vacant(e) = subst.entry(self.b.clone()) {
                e.insert((self.e3)(c, a));

                let s = format!(
                    "made progress: {:?} {} {:?} = {:?}",
                    &self.c, &self.op, &self.a, &self.b
                );
                dbg!(s);
                return true;
            }
        }
        false
    }
}
