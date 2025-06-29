use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, Clone)]
pub enum Domain {
    Range { min: u32, max: u32 },
    Set(BTreeSet<u32>),
    Top,
    Bottom,
}

impl Domain {
    pub fn intersect(&self, other: &Self) -> Self {
        use Domain::*;
        match (self, other) {
            (Top, x) | (x, Top) => x.clone(),
            (Set(s1), Set(s2)) => {
                let mut intersect = s1.intersection(s2).into_iter().peekable();
                if intersect.peek().is_none() {
                    return Bottom;
                }
                Set(BTreeSet::from_iter(intersect.cloned()))
            }

            (Range { min: s1, max: e1 }, Range { min: s2, max: e2 }) => {
                let min = *s1.min(s2);
                let max = *e1.max(e2);
                if max > min {
                    return Range { min, max };
                }
                Bottom
            }

            (Range { min: s, max: e }, Set(set)) | (Set(set), Range { min: s, max: e }) => {
                let i: BTreeSet<_> = set.range(s..=e).cloned().collect();
                if i.is_empty() {
                    Domain::Bottom
                } else {
                    Domain::Set(i)
                }
            }
            (Bottom, _) | (_, Bottom) => Bottom,
        }
    }

    pub fn is_singleton(&self) -> Option<u32> {
        match self {
            Domain::Range { min, max } if min == max => Some(*min),
            Domain::Set(s) if s.len() == 1 => s.iter().cloned().next(),
            _ => None,
        }
    }
    pub fn contains(&self, val: u32) -> bool {
        match self {
            Domain::Range { min, max } => (min..=max).contains(&&val),
            Domain::Set(s) => s.contains(&val),
            Domain::Top => true,
            Domain::Bottom => false,
        }
    }

    pub fn min(&self) -> Option<u32> {
        match self {
            Domain::Range { min, .. } => Some(*min),
            Domain::Set(s) => s.iter().next().copied(),
            _ => None,
        }
    }

    pub fn max(&self) -> Option<u32> {
        match self {
            Domain::Range { max, .. } => Some(*max),
            Domain::Set(s) => s.iter().next_back().copied(),
            _ => None,
        }
    }
    pub fn to_set(&self) -> Option<BTreeSet<u32>> {
        match self {
            Domain::Set(s) => Some(s.clone()),
            Domain::Range { min, max } if max - min <= 256 => Some((*min..=*max).collect()),
            Domain::Bottom => Some(BTreeSet::new()),
            _ => None,
        }
    }

    pub fn new_range(min: u32, max: u32) -> Self {
        Self::Range { min, max }
    }
}

fn range_cmp(a_min: u32, a_max: u32, b_min: u32, b_max: u32) -> Option<Ordering> {
    if a_min == b_min && a_max == b_max {
        Some(Ordering::Equal)
    } else if a_min >= b_min && a_max <= b_max {
        Some(Ordering::Less)
    } else if a_min <= b_min && a_max >= b_max {
        Some(Ordering::Greater)
    } else {
        None
    }
}

fn set_cmp(a: &BTreeSet<u32>, b: &BTreeSet<u32>) -> Option<Ordering> {
    if a == b {
        Some(Ordering::Equal)
    } else if a.is_subset(b) {
        Some(Ordering::Less)
    } else if a.is_superset(b) {
        Some(Ordering::Greater)
    } else {
        None
    }
}

impl PartialEq for Domain {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl Eq for Domain {}

impl PartialOrd for Domain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Domain::{Bottom, Range, Set, Top};

        match (self, other) {
            (Bottom, Bottom) | (Top, Top) => Some(Ordering::Equal),
            (Bottom, _) => Some(Ordering::Less),
            (_, Bottom) => Some(Ordering::Greater),
            (_, Top) => Some(Ordering::Less),
            (Top, _) => Some(Ordering::Greater),

            (Range { min: a1, max: b1 }, Range { min: a2, max: b2 }) => {
                range_cmp(*a1, *b1, *a2, *b2)
            }

            (Set(s1), Set(s2)) => set_cmp(s1, s2),

            (a, b) => match (a.to_set(), b.to_set()) {
                (Some(sa), Some(sb)) => set_cmp(&sa, &sb),
                _ => None,
            },
        }
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! two_real {
        ($a:expr,$b:expr) => {
            ($a.is_singleton(), $b.is_singleton())
        };
    }
}
