use std::path::PathBuf;

use thiserror::Error;

use crate::project::ProjectId;

#[derive(Error, Debug, PartialEq)]
pub enum AddProjectError {
    #[error("Path already added with id {0}")]
    PathAlreadyAdded(ProjectId),
    #[error("The dependency {0} was not found in the workspace")]
    DepedencyNotFound(ProjectId),
}

#[derive(Error, Debug, PartialEq)]
pub enum MarkProjectAsAffectedError {
    #[error("Project {0} not found")]
    ProjectNotFound(ProjectId),
}

#[derive(Error, Debug, PartialEq)]
pub enum BuildWorkspaceError {
    #[error("Error while adding project {0}: {1}")]
    ErrorWhileAddingProject(PathBuf, AddProjectError),
    #[error("The project declaration for the path {0} was not found")]
    ProjectDeclarationNotFound(PathBuf),
    #[error("A cyclic dependency with the path {0:?} was found")]
    CyclicDependencyFound(Vec<PathBuf>),
}
