use crate::{
    constraint::{Constraint, ConstraintDescription},
    constraint_err,
    domain::Domain,
    error::{Progress, TraceBuilder, UnifyError},
    term::{Subst, Term},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LessThanConstraint {
    lhs: Term,
    rhs: Term,
}

impl LessThanConstraint {
    pub fn boxed(lhs: Term, rhs: Term) -> Box<Self> {
        Box::new(Self { lhs, rhs })
    }
}

impl Constraint for LessThanConstraint {
    fn hold(&self, subst: &Subst) -> bool {
        let Some((lhs, rhs)) = subst.get_pair(&self.lhs, &self.rhs) else {
            return true;
        };

        match (lhs.is_singleton(), rhs.is_singleton()) {
            (Some(left), Some(right)) => left < right,
            _ => true,
        }
    }

    fn describe(&self) -> ConstraintDescription {
        ConstraintDescription::new(
            "lt".to_string(),
            &[self.lhs.clone(), self.rhs.clone()],
            format!("{} < {}", self.lhs, self.rhs),
        )
    }

    fn make_progress(
        &self,
        subst: &mut Subst,
        trace: &mut TraceBuilder,
    ) -> Result<Progress, UnifyError> {
        match (subst.get(&self.lhs), subst.get(&self.rhs)) {
            (Some(dl), Some(dr)) => match (dl.is_singleton(), dr.is_singleton()) {
                (Some(a), Some(b)) if a < b => Ok(Progress::Made),
                (Some(a), Some(b)) => constraint_err!(trace, self, "{a} < {b} failed"),
                _ => Ok(Progress::Stalled),
            },
            (Some(dl), None) => {
                if let Some(max) = dl.min() {
                    let new = Domain::new_range(max + 1, u32::MAX);
                    if subst.bind(self.rhs.clone(), new) {
                        return Ok(Progress::Made);
                    }
                }
                Ok(Progress::Stalled)
            }
            (None, Some(dr)) => {
                if let Some(min) = dr.max() {
                    let new = Domain::new_range(0, min - 1);
                    if subst.bind(self.rhs.clone(), new) {
                        return Ok(Progress::Made);
                    }
                }

                Ok(Progress::Stalled)
            }

            // todo rest of the arms
            _ => Ok(Progress::Stalled),
        }
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

// impl Constraint for GreaterThanConstraint {
//     fn hold(&self, subst: &Subst) -> bool {
//         match (subst.get(&self.t1), subst.get(&self.t2)) {
//             (Some(a), Some(b)) => {
//                 eprintln!("holding? {a} > {b}");
//                 a > b
//             }
//             (a, b) => {
//                 eprintln!("holding? {a:?} > {b:?}");
//                 true
//             }
//         }
//     }

//     fn make_progress(&self, _subst: &mut Subst) -> bool {
//         false
//     }
// }
