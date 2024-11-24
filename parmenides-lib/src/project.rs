use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ProjectId(usize);

impl ProjectId {
    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn into_inner(&self) -> usize {
        self.0
    }
}

impl Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.into_inner())
    }
}

#[derive(Debug)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub dependencies: Option<Vec<ProjectId>>,
    pub dependents: Vec<ProjectId>,
    pub affected: bool,
}

impl Project {
    pub fn new(path: PathBuf, name: String, dependencies: Option<Vec<ProjectId>>) -> Self {
        Self {
            path,
            name,
            dependencies,
            dependents: vec![],
            affected: false,
        }
    }

    pub fn add_dependent(&mut self, id: ProjectId) {
        self.dependents.push(id);
    }
}

impl<'a> Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project {} {:?}", self.name, self.path)
    }
}
