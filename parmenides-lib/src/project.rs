use std::{fmt::Display, path::Path};

use nutype::nutype;

#[nutype(derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy))]
pub struct ProjectId(usize);

impl Display for ProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.into_inner())
    }
}

#[derive(Debug)]
pub struct Project<'a> {
    pub path: &'a Path,
    pub name: &'a str,
    pub dependencies: Vec<ProjectId>,
    pub dependents: Vec<ProjectId>,
}

impl<'a> Project<'a> {
    pub fn new(path: &'a Path, name: &'a str, dependencies: Vec<ProjectId>) -> Self {
        Self {
            path,
            name,
            dependencies,
            dependents: vec![],
        }
    }

    pub fn add_dependent(&mut self, index: ProjectId) {
        self.dependents.push(index);
    }
}

impl<'a> Display for Project<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project {} {:?}", self.name, self.path)
    }
}
