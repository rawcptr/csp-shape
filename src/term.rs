use std::fmt::Display;

use crate::CowStr;
pub type VarId = usize;
pub type Val = i32;

pub struct VarGen {
    next_id: VarId,
}

impl VarGen {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    pub fn fresh(&mut self, name: Option<CowStr>) -> Term {
        let id = self.next_id;
        self.next_id += 1;
        Term::Var { name, id }
    }
}

impl Default for VarGen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Val(Val),
    Var { name: Option<CowStr>, id: usize },
}

impl Term {
    pub fn no_name(&self) -> String {
        match self {
            Term::Val(x) => x.to_string(),
            Term::Var { id, .. } => format!("#{id}"),
        }
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Val(v) => write!(f, "{v}"),
            Term::Var { name, id } => {
                write!(
                    f,
                    "{id}{}",
                    name.as_ref()
                        .map(|f| format!(" (named: {f})"))
                        .unwrap_or_default()
                )
            }
        }
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! val {
        ($n:expr) => {
            Term::Val($n)
        };
    }

    #[macro_export]
    macro_rules! fresh_var {
        ($var_gen:expr $(,)*) => {
            $var_gen.fresh(None)
        };
        ($var_gen:expr, $name:expr $(,)*) => {
            $var_gen.fresh(Some($name.into()))
        };
    }
}
