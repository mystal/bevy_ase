use std::{fmt, path::PathBuf};

use asefile::AsepriteFile;
use bevy::utils::HashMap;

/// Unique identifier for an Aseprite file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AseId(u32);
impl AseId {
    /// Creates a new AseId over a u32 value.
    /// During asset loading, AseId values are generated by wrapping the file index.
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
    /// Returns a reference to the AseId's underlying u32 value.
    pub fn inner(&self) -> &u32 {
        &self.0
    }
}
impl fmt::Display for AseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AseId({})", self.0)
    }
}

pub(crate) struct AsepriteFileWithId {
    pub(crate) id: AseId,
    pub(crate) path: PathBuf,
    pub(crate) file: AsepriteFile,
}

pub(crate) struct AsesById(HashMap<AseId, AsepriteFileWithId>);
impl AsesById {
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, AseId, AsepriteFileWithId> {
        self.0.iter()
    }
}
impl From<Vec<(PathBuf, AsepriteFile)>> for AsesById {
    fn from(vec: Vec<(PathBuf, AsepriteFile)>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(idx, (path, file))| {
                    let id = AseId::new(idx as u32);
                    let value = AsepriteFileWithId { id, path, file };
                    (id, value)
                })
                .collect(),
        )
    }
}