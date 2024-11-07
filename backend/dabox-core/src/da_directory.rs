use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, sqlx::Type)]
#[sqlx(transparent)]
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
