use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use git2::Repository;

use super::DiffEngine;

pub struct GitDiffEngine;

impl DiffEngine for GitDiffEngine {
    fn get_affected_paths<P>(path: P, from: String, to: String) -> Result<HashSet<PathBuf>, String>
    where
        P: AsRef<std::path::Path>,
    {
        get_affected_files_git(path.as_ref(), &from, &to).map_err(|err| err.to_string())
    }
}

fn get_affected_files_git(
    repo_path: &Path,
    from: &str,
    to: &str,
) -> Result<HashSet<PathBuf>, git2::Error> {
    let repo = Repository::open(repo_path)?;

    let commit_from = repo.revparse_single(from)?;
    let tree_from = commit_from.peel_to_tree()?;

    let commit_to = repo.revparse_single(to)?;
    let tree_to = commit_to.peel_to_tree()?;

    let diff = repo.diff_tree_to_tree(Some(&tree_from), Some(&tree_to), None)?;

    let mut affected_paths = HashSet::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                affected_paths.insert(repo_path.join(path));
            }
            true
        },
        None,
        None,
        None,
    )?;

    Ok(affected_paths)
}
