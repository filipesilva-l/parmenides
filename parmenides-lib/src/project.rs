use std::fmt::Display;
use std::path::PathBuf;

/// The unique identifier for a project within a workspace.
///
/// Each project added to a workspace is assigned a `ProjectId`. It is used to track
/// dependencies, dependents, and for efficient project lookup.
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

/// Represents an individual project in a workspace.
///
/// A `Project` encapsulates the project's metadata, such as its path, name, dependencies,
/// and dependents. It also tracks whether the project is affected by a change.
#[derive(Debug)]
pub struct Project {
    /// The file path of the project.
    pub path: PathBuf,

    /// The human-readable name of the project.
    pub name: String,

    /// The dependencies of this project, represented as a list of `ProjectId`s.
    ///
    /// `None` indicates that the project has no dependencies.
    pub dependencies: Option<Vec<ProjectId>>,

    /// The dependents of this project, represented as a list of `ProjectId`s.
    ///
    /// This list is automatically populated when other projects declare this project as a dependency.
    pub dependents: Vec<ProjectId>,

    /// Indicates whether this project is affected by a change.
    ///
    /// This field is useful for tracking which projects need to be rebuilt or tested after a change.
    pub affected: bool,
}

impl Project {
    pub(crate) fn new(path: PathBuf, name: String, dependencies: Option<Vec<ProjectId>>) -> Self {
        Self {
            path,
            name,
            dependencies,
            dependents: vec![],
            affected: false,
        }
    }

    pub(crate) fn add_dependent(&mut self, id: ProjectId) {
        self.dependents.push(id);
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Project {} {:?}", self.name, self.path)
    }
}
