use thiserror::Error;

use crate::project::ProjectId;

#[derive(Error, Debug, PartialEq)]
pub enum AddProjectError {
    #[error("Path already added with id {0}")]
    PathAlreadyAdded(ProjectId),
    #[error("The dependency {0} was not found in the workspace")]
    DepedencyNotFound(ProjectId),
}
