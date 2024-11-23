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

    pub fn get_project(&self, id: ProjectId) -> Option<&Project> {
        self.arena.get(id.into_inner())
    }
}

impl<'a> Default for Workspace<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;
    use crate::project::Project;
    use std::path::Path;

    #[test]
    pub fn when_adding_project_should_add() {
        let path = Path::new("/home/test/project");
        let name = "test";

        let mut workspace = Workspace::new();

        let project_id = workspace
            .add_project(Project::new(path, name, None))
            .unwrap();

        let project = workspace.get_project(project_id).unwrap();

        assert_eq!(project.path, path);
        assert_eq!(project.name, name);
    }
}
