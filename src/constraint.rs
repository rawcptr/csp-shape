use crate::term::Subst;

pub trait Constraint {
    // ideally both would return UnifyResult<(), UnifyError> or something
    fn hold(&self, subst: &Subst) -> bool;
    fn make_progress(&self, subst: &mut Subst) -> bool;
}
pub struct Constraints(Vec<Box<dyn Constraint>>);

impl Constraints {
    pub fn new(constraints: Vec<Box<dyn Constraint>>) -> Constraints {
        Self(constraints)
    }
    
    pub fn solve(&self, subst: &mut Subst) {
        loop {
            let mut made_progress = false;

            for constraint in &self.0 {
                made_progress |= constraint.make_progress(subst)
            }

            if !made_progress {
                break; // we're done!
            }
        }
    }
}
