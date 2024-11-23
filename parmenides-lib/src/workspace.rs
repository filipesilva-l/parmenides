use std::{collections::HashMap, path::Path};

use crate::{
    errors::AddProjectError,
    project::{Project, ProjectId},
};

pub struct Workspace<'a> {
    arena: Vec<Project<'a>>,
    hash: HashMap<&'a Path, ProjectId>,
}

impl<'a> Workspace<'a> {
    pub fn new() -> Self {
        Self {
            arena: vec![],
            hash: HashMap::new(),
        }
    }

    pub fn add_project(&mut self, project: Project<'a>) -> Result<ProjectId, AddProjectError> {
        let id = ProjectId::new(self.arena.len());

        let existing_project = self.hash.insert(project.path, id);

        if let Some(existing_id) = existing_project {
            return Err(AddProjectError::ProjectAlreadyAdded(existing_id));
        }

        self.arena.push(project);

        Ok(id)
    }
}

impl<'a> Default for Workspace<'a> {
    fn default() -> Self {
        Self::new()
    }
}
