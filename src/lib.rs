use std::borrow::Cow;

use crate::error::UnifyError;

pub mod domain;
pub mod error;
pub mod term;

pub(crate) type CowStr = Cow<'static, str>;
pub type Result<T> = std::result::Result<T, UnifyError>;
