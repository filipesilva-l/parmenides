use thiserror::Error;

use crate::project::ProjectId;

#[derive(Error, Debug)]
pub enum AddProjectError {
    #[error("Project already added with id {0}")]
    ProjectAlreadyAdded(ProjectId),
}
