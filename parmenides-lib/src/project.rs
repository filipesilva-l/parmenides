use std::{fmt::Display, path::Path};

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
pub struct Project<'a> {
    pub path: &'a Path,
    pub name: &'a str,
    pub dependencies: Option<Vec<ProjectId>>,
    pub dependents: Vec<ProjectId>,
}

impl<'a> Project<'a> {
    pub fn new(path: &'a Path, name: &'a str, dependencies: Option<Vec<ProjectId>>) -> Self {
        Self {
            path,
            name,
            dependencies,
            dependents: vec![],
        }
    }

    pub fn add_dependent(&mut self, id: ProjectId) {
        self.dependents.push(id);
    }
}

impl<'a> Display for Project<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project {} {:?}", self.name, self.path)
    }
}
