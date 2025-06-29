use std::fmt::Display;

use crate::{
    error::{Progress, TraceBuilder, UnifyError},
    term::{Subst, Term},
};

pub trait Constraint {
    // ideally both would return UnifyResult<(), UnifyError> or something
    fn hold(&self, subst: &Subst) -> bool;
    fn make_progress(
        &self,
        subst: &mut Subst,
        trace: &mut TraceBuilder,
    ) -> Result<Progress, UnifyError>;
    fn describe(&self) -> ConstraintDescription;
}
pub struct Constraints(Vec<Box<dyn Constraint>>);

impl Constraints {
    pub fn new(constraints: Vec<Box<dyn Constraint>>) -> Constraints {
        Self(constraints)
    }

    pub fn solve(&self, subst: &mut Subst) {
        for _ in 0..100 {
            let mut made_progress = false;

            for constraint in &self.0 {
                made_progress |= constraint.make_progress(subst)
            }

            if !made_progress {
                for (idx, constraint) in self.0.iter().enumerate() {
                    let idx = idx + 1;
                    let holds = constraint.hold(subst);

                    if !holds {
                        eprintln!("constraint {idx} failed");
                        break;
                    }
                }
                break;
            }
        }
        panic!("recursion hit");
    }
}

#[derive(Debug, Clone)]
pub struct ConstraintDescription {
    typename: String,
    terms: Vec<Term>,
    details: String, // human readable string
}

impl Display for ConstraintDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "constraint {}: {}", self.typename, self.details)
    }
}

impl ConstraintDescription {
    pub fn new(typename: String, terms: &[Term], details: String) -> Self {
        Self {
            typename,
            terms: terms.to_vec(),
            details,
        }
    }

    pub fn typename(&self) -> &str {
        &self.typename
    }

    pub fn terms(&self) -> &[Term] {
        &self.terms
    }

    pub fn details(&self) -> &str {
        &self.details
    }
}
