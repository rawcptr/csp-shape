use std::fmt::{self, Display};

use crate::{
    CowStr,
    domain::Domain,
    term::{Term, Val},
};
use yansi::Paint;

const SPACER: &str = "  ";
#[derive(Clone)]
pub enum UnifyError {
    Csp(CspError),
    Solver(String),
}

impl From<CspError> for UnifyError {
    fn from(value: CspError) -> Self {
        Self::Csp(value)
    }
}

impl std::error::Error for UnifyError {}

impl fmt::Debug for UnifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Csp(arg0) => write!(f, "{arg0:?}"),
            Self::Solver(arg0) => write!(f, "{} {}", "solver error:".red().bold(), arg0),
        }
    }
}

impl Display for UnifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnifyError::Csp(err) => write!(f, "{err}"),
            UnifyError::Solver(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Clone)]
pub struct CspError {
    constraint: &'static str,
    trace: Vec<TraceFrame>,
    reason: Option<String>,
}

impl CspError {
    pub fn new(constraint: &'static str, trace: Vec<TraceFrame>, reason: Option<String>) -> Self {
        Self {
            constraint,
            reason,
            trace,
        }
    }
}

impl fmt::Display for CspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Constraint '{}', failed: {}",
            self.constraint,
            self.reason.as_deref().unwrap_or("no reason provided")
        )
    }
}

impl fmt::Debug for CspError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            "constraint failed:".bold().red(),
            self.constraint
        )?;
        writeln!(f, "{}", "trace:".blue())?;
        for entry in self.trace.iter() {
            let disp = format!("{entry:?}");
            for line in disp.lines() {
                writeln!(f, "{SPACER}{SPACER}{line}")?;
            }
        }
        write!(
            f,
            "{} {}",
            "reason:".green(),
            self.reason.as_deref().unwrap_or("not provided")
        )
    }
}

#[derive(Clone)]
pub enum TraceFrame {
    Branched {
        var: Term,
        value: Val,
    },
    Constrained {
        constraint: CowStr,
        domains: Vec<(Term, Domain)>,
    },
    Backtracked {
        var: Term,
        failed_value: Val,
    },
}

impl Display for TraceFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceFrame::Branched { var, value } => write!(f, "var: {var} value: {value}"),
            TraceFrame::Constrained {
                constraint,
                domains,
            } => write!(f, "constraint: {constraint}, domains: {domains:?}"),
            TraceFrame::Backtracked { var, failed_value } => {
                write!(f, "var {var}, failed value: {failed_value}")
            }
        }
    }
}

impl std::fmt::Debug for TraceFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Branched { var, value } => write!(f, "\u{2192} branched {var} = {value}"),
            Self::Constrained {
                constraint,
                domains,
            } => {
                let constr = format!("\u{2192} `{constraint}`");
                let offsets: Vec<_> = domains
                    .iter()
                    .filter_map(|d| constr.find(&d.0.no_name()))
                    .map(|f| f - '\u{2192}'.len_utf8() + 1)
                    .collect();

                writeln!(f, "{constr}")?;
                if offsets.is_empty() {
                    return Ok(());
                }

                for (i, (term, domain)) in domains.iter().enumerate().rev() {
                    let mut disp = vec![' '; offsets[i]];
                    let rest = offsets.iter().take(i);

                    // vertical bar
                    if rest.len() > 0 {
                        for j in rest {
                            disp[*j] = '\u{2502}';
                        }
                    }

                    writeln!(
                        f,
                        "{}╰─ {term} ∈ {domain:?}",
                        disp.into_iter().collect::<String>()
                    )?;
                }

                Ok(())
            }
            Self::Backtracked { var, failed_value } => {
                write!(f, "\u{2192} backtracked for {var} \u{2260} {failed_value}")
            }
        }
    }
}

pub(crate) mod macros {
    #[macro_export]
    macro_rules! constraint_err {
        ($cons:expr, $trace:expr, $reason:expr $(,)*) => {
            $crate::error::CspError::new($cons, $trace, Some($reason.into()))
        };
        ($cons:expr, $trace:expr $(,)* ) => {
            CspError {
                constraint: $cons,
                trace: $trace,
                reason: None,
            }
        };
    }

    #[macro_export]
    macro_rules! csp_bail {
        ($cons:expr, $trace:expr, $reason:expr $(,)*) => {
            return Err($crate::constraint_err!($cons, $trace, $reason).into());
        };
        ($cons:expr, $trace:expr $(,)*) => {
            return Err($crate::constraint_err!($cons, $trace).into());
        };
    }
}
