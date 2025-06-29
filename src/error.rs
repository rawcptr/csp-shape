use std::fmt::Display;

use crate::constraint::unify_v2::ConstraintDescription;

#[derive(Debug, Clone, Copy)]
pub enum Progress {
    Made,
    Stalled,
}

#[derive(Debug, Clone)]
pub struct ConstraintError {
    pub description: ConstraintDescription,
    pub reason: String,
    pub trace: Vec<ConstraintDescription>,
}

impl Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.description, self.reason)?;
        for trace in &self.trace {
            writeln!(f, "trace:\t{}", trace)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum UnifyError {
    ConstraintViolation(ConstraintError),
    SolverError { message: String },
}

impl From<ConstraintError> for UnifyError {
    fn from(value: ConstraintError) -> Self {
        Self::ConstraintViolation(value)
    }
}
