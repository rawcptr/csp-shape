use crate::{
    constraint::{Constraint, ConstraintDescription},
    constraint_err,
    error::{Progress, TraceBuilder, UnifyError},
    term::{Subst, Term},
};

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
            (Some(a), Some(b)) => {
                eprintln!("holding? {a} == {b}");
                a == b
            }
            (a, b) => {
                eprintln!("holding? {a:?} == {b:?}");
                true
            }
        }
    }

    fn make_progress(
        &self,
        subst: &mut Subst,
        trace: &mut TraceBuilder,
    ) -> Result<Progress, UnifyError> {
        if let (Some(a), Some(b)) = (subst.get(&self.t1), subst.get(&self.t2)) {
            if a != b {
                return constraint_err!(trace, self, "conflicting bindings: {a} != {b}");
            }
        }

        match (&self.t1, &self.t2) {
            // X = Y case - need to check if either is already assigned
            (Term::Var(x), Term::Var(y)) => {
                if subst.propagate(&Term::Var(x), &Term::Var(y))
                    || subst.propagate(&Term::Var(y), &Term::Var(x))
                {
                    return Ok(Progress::Made);
                }
                Ok(Progress::Stalled)
            }

            (Term::Var(x), Term::Val(v)) | (Term::Val(v), Term::Var(x)) => {
                if subst.propagate(&Term::Val(*v), &Term::Var(x)) {
                    return Ok(Progress::Made);
                };
                Ok(Progress::Stalled)
            }

            // 5 = 5 case - nothing to do
            (_, _) => Ok(Progress::Stalled),
        }
    }

    fn describe(&self) -> ConstraintDescription {
        ConstraintDescription::new(
            "equality".to_string(),
            &[self.t1.clone(), self.t2.clone()],
            format!("{} = {}", self.t1, self.t2),
        )
    }
}
