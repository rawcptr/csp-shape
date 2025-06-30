use std::fmt::Display;

use crate::constraint::ConstraintDescription;

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

pub struct TraceBuilder {
    trace: Vec<ConstraintDescription>,
}

impl TraceBuilder {
    pub fn new() -> Self {
        Self { trace: Vec::new() }
    }
    pub fn current_trace(&self) -> &[ConstraintDescription] {
        &self.trace
    }
    pub fn with_context<T>(
        &mut self,
        desc: ConstraintDescription,
        f: impl FnOnce(&mut Self) -> T,
    ) -> T {
        self.trace.push(desc);
        let result = f(self);
        self.trace.pop();
        result
    }

    pub fn push(&mut self, desc: ConstraintDescription) {
        self.trace.push(desc)
    }
    pub fn pop(&mut self) -> Option<ConstraintDescription> {
        self.trace.pop()
    }
    pub fn create_constraint_error(
        &self,
        description: ConstraintDescription,
        reason: String,
    ) -> ConstraintError {
        ConstraintError {
            description,
            reason,
            trace: self.trace.clone(),
        }
    }
}

pub mod macros {

    #[macro_export]
    macro_rules! constraint_err {
    ($trace:expr, $cons:expr, $($msg:tt)+) => {
        Err($trace.create_constraint_error(
            $cons.describe(),
            format!($($msg)+)
        ).into())
    };
}

    pub use constraint_err;
}
