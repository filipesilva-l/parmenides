use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

mod git;

pub trait DiffEngine {
    fn get_affected_paths<P>(path: P, from: String, to: String) -> Result<HashSet<PathBuf>, String>
    where
        P: AsRef<Path>;
}
