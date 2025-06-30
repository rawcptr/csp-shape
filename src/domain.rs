use std::{collections::BTreeSet, sync::Arc};

use crate::term::Val;

#[derive(Clone)]
pub enum Domain {
    Single(Val),
    Range { min: Val, max: Val },
    Set(Arc<BTreeSet<Val>>),
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cardinality {
    Finite(usize),
    Infinite,
}

impl From<Cardinality> for Option<usize> {
    fn from(value: Cardinality) -> Self {
        match value {
            Cardinality::Finite(v) => Some(v),
            Cardinality::Infinite => None,
        }
    }
}

impl Domain {
    pub fn refine(&self, other: &Self) -> Self {
        use Domain::*;
        match (self, other) {
            (x, Top) | (Top, x) => x.clone(),
            (_, Bottom) | (Bottom, _) => Bottom,
            (Set(set1), Set(set2)) => set_from(set1.intersection(set2).cloned().peekable()),
            (Single(x), Single(y)) if x == y => Self::Single(*x),
            (Single(s), Range { min, max }) | (Range { min, max }, Single(s))
                if s >= min && s <= max =>
            {
                Self::Single(*s)
            }
            (Single(x), Set(btree_set)) | (Set(btree_set), Single(x)) if btree_set.contains(x) => {
                Self::Single(*x)
            }
            (Range { min: mn1, max: mx1 }, Range { min: mn2, max: mx2 }) => {
                let min = *mn1.max(mn2);
                let max = *mx1.min(mx2);
                match min.cmp(&max) {
                    std::cmp::Ordering::Less => Range { min, max },
                    std::cmp::Ordering::Equal => Single(min),
                    std::cmp::Ordering::Greater => Bottom,
                }
            }
            (Range { min, max }, Set(btree_set)) | (Set(btree_set), Range { min, max }) => {
                set_from(btree_set.iter().filter(|x| x >= &min && x <= &max).cloned())
            }
            (lhs, rhs) => panic!("unhandled domain refinement: {lhs:?} ∩ {rhs:?}"),
        }
    }

    pub fn contains(&self, val: Val) -> bool {
        match self {
            Domain::Single(s) if *s == val => true,
            Domain::Range { min, max } if *min <= val && *max >= val => true,
            Domain::Set(btree_set) => btree_set.contains(&val),
            _ => false,
        }
    }

    pub fn cardinality(&self) -> Cardinality {
        use Cardinality::{Finite, Infinite};
        match self {
            Domain::Single(_) => Finite(1),
            Domain::Range { min, max } if min > max => Finite(0),
            Domain::Range { min, max } => Finite((max - min + 1).unsigned_abs() as usize),
            Domain::Set(btree_set) => Finite(btree_set.len()),
            Domain::Top => Infinite,
            Domain::Bottom => Finite(0),
        }
    }

    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(_))
    }

    pub fn is_top(&self) -> bool {
        matches!(self, Self::Top)
    }

    pub fn is_bottom(&self) -> bool {
        matches!(self, Self::Bottom)
    }
}

fn set_from<I: IntoIterator<Item = Val>>(iter: I) -> Domain {
    let set: BTreeSet<_> = iter.into_iter().collect();
    match set.len() {
        0 => Domain::Bottom,
        1 => Domain::Single(set.iter().next().cloned().unwrap()),
        _ => Domain::Set(Arc::new(set)),
    }
}

impl std::fmt::Debug for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Domain::Single(v) => write!(f, "{v}"),
            Domain::Range { min, max } => write!(f, "[{}..={}]", min, max),
            Domain::Set(btree_set) => {
                write!(
                    f,
                    "{{{}",
                    btree_set
                        .iter()
                        .take(5)
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
                if btree_set.len() > 5 {
                    write!(f, ", ... ({} total)", btree_set.len())?;
                }
                write!(f, "}}")
            }
            Domain::Top => write!(f, "ᴛ"),
            Domain::Bottom => write!(f, "∅"),
        }
    }
}
