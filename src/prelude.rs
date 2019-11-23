pub(crate) use crate::mat2::Mat2;
pub(crate) use crate::parsers;
pub(crate) use crate::vec2::Vec2us;
pub(crate) use anyhow::{anyhow, Result};
pub(crate) use arrayvec::ArrayVec;
pub(crate) use itertools::Itertools;
pub(crate) use nom::IResult;
pub(crate) use std::collections::{HashMap, HashSet};
pub(crate) use std::fmt::{self, Debug, Display, Formatter};
pub(crate) use std::iter::{repeat, FromIterator, Iterator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parts(pub String, pub String);
pub trait ToParts {
    fn to_parts(&self) -> Parts;
}

impl<A, B> ToParts for (A, B)
where
    A: std::string::ToString,
    B: std::string::ToString,
{
    fn to_parts(&self) -> Parts {
        Parts(self.0.to_string(), self.1.to_string())
    }
}

impl Display for Parts {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use crossterm::style::{Colorize, StyledContent};
        use fmt::Write;
        <StyledContent<&'static str> as Display>::fmt(&"pt1".yellow(), f)?;
        f.write_char(if self.0.contains('\n') { '\n' } else { ' ' })?;
        <String as Display>::fmt(&self.0, f)?;
        f.write_char('\n')?;
        <StyledContent<&'static str> as Display>::fmt(&"pt2".yellow(), f)?;
        f.write_char(if self.1.contains('\n') { '\n' } else { ' ' })?;
        <String as Display>::fmt(&self.1, f)
    }
}
