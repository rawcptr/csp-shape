use crate::constraint::unify_v2::ConstraintDescription;

#[derive(Debug)]
pub enum Progress {
    Made,
    Stalled,
}

pub struct ConstraintError {
    pub description: ConstraintDescription,
    pub reason: String,
    pub trace: 

}
