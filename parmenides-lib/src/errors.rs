//! # Errors
//!
//! This module defines the error types used throughout `parmenides-lib` to handle various failure
//! scenarios gracefully. Each error type is specific to a part of the library and is designed to
//! provide detailed and actionable error messages.
use std::path::PathBuf;

use thiserror::Error;

use crate::project::ProjectId;

/// Errors that can occur while adding a project to the [`crate::workspace::Workspace`].
#[derive(Error, Debug, PartialEq)]
pub enum AddProjectError {
    /// Indicates that a project with the same path has already been added to the workspace.
    #[error("Path already added with id {0}")]
    PathAlreadyAdded(ProjectId),
    /// Indicates that a dependency specified for a project could not be found in the workspace.
    #[error("The dependency {0} was not found in the workspace")]
    DepedencyNotFound(ProjectId),
}

/// Errors that can occur while marking a project as affected in the [`crate::workspace::Workspace`].
#[derive(Error, Debug, PartialEq)]
pub enum MarkProjectAsAffectedError {
    /// Indicates that the specified project could not be found in the workspace.
    #[error("Project {0} not found")]
    ProjectNotFound(ProjectId),
}

/// Errors that can occur while building a [`crate::workspace::Workspace`] from a
/// [`crate::declarations::WorkspaceDeclaration`].
#[derive(Error, Debug, PartialEq)]
pub enum BuildWorkspaceError {
    /// Indicates that adding a project to the workspace failed.
    #[error("Error while adding project {0}: {1}")]
    ErrorWhileAddingProject(PathBuf, AddProjectError),
    /// Indicates that a project declaration for a specific path was not found.
    #[error("The project declaration for the path {0} was not found")]
    ProjectDeclarationNotFound(PathBuf),
    /// Indicates that a cyclic dependency was detected while resolving project dependencies.
    #[error("A cyclic dependency with the path {0:?} was found")]
    CyclicDependencyFound(Vec<PathBuf>),
}
