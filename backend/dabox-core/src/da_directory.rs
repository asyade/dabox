use crate::prelude::*;
use std::fmt::{Display, Formatter};

#[cfg(feature = "database")]
#[derive(
    Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct DaDirectorySid(pub(crate) i64);

#[cfg(not(feature = "database"))]
#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct DaDirectorySid(pub(crate) i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaDirectory {
    /// The unique identifier of the directory
    pub sid: DaDirectorySid,
    /// The name of the directory
    pub name: String,
    /// The parent directory's unique identifier
    pub parent_sid: Option<DaDirectorySid>,
    /// The children directories
    pub children: Vec<DaDirectory>,
    /// The depth of the directory in the hierarchy
    pub depth: u32,
}

impl DaDirectory {
    fn fmt_with_children(&self, f: &mut Formatter<'_>, depth: usize) -> std::fmt::Result {
        writeln!(
            f,
            "| {:>indent$}",
            self.name,
            indent = (depth + self.name.chars().count())
        )?;
        for child in self.children.iter() {
            child.fmt_with_children(f, depth + 1)?;
        }
        Ok(())
    }
}

impl Display for DaDirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-")?;
        self.fmt_with_children(f, 0)?;
        writeln!(f, "-")
    }
}

impl Display for DaDirectorySid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
