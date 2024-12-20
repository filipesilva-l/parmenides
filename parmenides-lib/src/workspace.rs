use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use crate::{
    errors::{AddProjectError, MarkProjectAsAffectedError},
    project::{Project, ProjectId},
};

#[derive(Debug)]
pub struct Workspace {
    arena: Vec<Project>,
    hash: HashMap<PathBuf, ProjectId>,
}

impl Workspace {
    pub(crate) fn new() -> Self {
        Self {
            arena: vec![],
            hash: HashMap::new(),
        }
    }

    pub(crate) fn add_project(&mut self, project: Project) -> Result<ProjectId, AddProjectError> {
        let id = ProjectId::new(self.arena.len());

        if let Some(existing_id) = self.hash.insert(project.path.clone(), id) {
            return Err(AddProjectError::PathAlreadyAdded(existing_id));
        }

        if let Some(dependencies) = &project.dependencies {
            for dependency in dependencies {
                let project = self
                    .arena
                    .get_mut(dependency.into_inner())
                    .ok_or(AddProjectError::DepedencyNotFound(*dependency))?;

                project.add_dependent(id)
            }
        }

        self.arena.push(project);

        Ok(id)
    }

    pub fn get_id_by_path<P>(&self, path: &P) -> Option<ProjectId>
    where
        P: AsRef<Path>,
    {
        self.hash.get(path.as_ref()).copied()
    }

    pub fn len(&self) -> usize {
        self.arena.len()
    }

    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    pub fn get_project(&self, id: ProjectId) -> Option<&Project> {
        self.arena.get(id.into_inner())
    }

    pub fn get_project_by_path<P>(&self, path: &P) -> Option<&Project>
    where
        P: AsRef<Path>,
    {
        self.get_id_by_path(path)
            .and_then(|id| self.get_project(id))
    }

    pub fn mark_project_as_affected(
        &mut self,
        id: ProjectId,
    ) -> Result<(), MarkProjectAsAffectedError> {
        let mut stack = vec![id];

        while let Some(current_id) = stack.pop() {
            let project = self
                .arena
                .get_mut(current_id.into_inner())
                .ok_or(MarkProjectAsAffectedError::ProjectNotFound(current_id))?;

            if !project.affected {
                project.affected = true;
                stack.extend(&project.dependents);
            }
        }

        Ok(())
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Workspace;
    use crate::{
        errors::AddProjectError,
        project::{Project, ProjectId},
    };
    use std::path::Path;

    #[test]
    pub fn when_adding_project_should_add() {
        let path = Path::new("/home/test/project");
        let name = "test";

        let mut workspace = Workspace::new();

        let project_id = workspace
            .add_project(Project::new(path.to_owned(), name.to_owned(), None))
            .unwrap();

        let project = workspace.get_project(project_id).unwrap();

        assert_eq!(project.path, path);
        assert_eq!(project.name, name);
    }

    #[test]
    pub fn when_adding_the_same_project_twice_should_return_error() {
        let path = Path::new("/home/test/project");
        let name = "test";

        let mut workspace = Workspace::new();

        let id = workspace
            .add_project(Project::new(path.to_owned(), name.to_owned(), None))
            .unwrap();

        let error = workspace
            .add_project(Project::new(path.to_owned(), name.to_owned(), None))
            .unwrap_err();

        assert_eq!(AddProjectError::PathAlreadyAdded(id), error);
    }

    #[test]
    pub fn when_adding_project_dependent_should_update_dependents() {
        let mut workspace = Workspace::new();

        let core_id = workspace
            .add_project(Project::new(
                Path::new("/home/test/core").to_owned(),
                "core".to_owned(),
                None,
            ))
            .unwrap();

        let dependent_id = workspace
            .add_project(Project::new(
                Path::new("/home/test/dependent").to_owned(),
                "dependent".to_owned(),
                Some(vec![core_id]),
            ))
            .unwrap();

        let core = workspace.get_project(core_id).unwrap();

        assert!(!core.dependents.is_empty());
        assert_eq!(core.dependents, vec![dependent_id]);

        let dependent = workspace.get_project(dependent_id).unwrap();

        assert!(dependent.dependents.is_empty());
        assert!(dependent.dependencies.is_some());
        assert_eq!(dependent.dependencies, Some(vec![core_id]));
    }

    #[test]
    pub fn when_dependency_doesnt_exist_should_return_error() {
        let path = Path::new("/home/test/project");
        let name = "test";

        let mut workspace = Workspace::new();

        let dependency_id = ProjectId::new(12);

        let error = workspace
            .add_project(Project::new(
                path.to_owned(),
                name.to_owned(),
                Some(vec![dependency_id]),
            ))
            .unwrap_err();

        assert_eq!(AddProjectError::DepedencyNotFound(dependency_id), error);
    }

    #[test]
    pub fn when_marking_project_as_affected_should_mark_dependents_too() {
        let mut workspace = Workspace::new();

        let core_id = workspace
            .add_project(Project::new(
                Path::new("/home/test/core").to_owned(),
                "core".to_owned(),
                None,
            ))
            .unwrap();

        let dependent_id = workspace
            .add_project(Project::new(
                Path::new("/home/test/dependent").to_owned(),
                "dependent".to_owned(),
                Some(vec![core_id]),
            ))
            .unwrap();

        workspace.mark_project_as_affected(core_id).unwrap();

        let core = workspace.get_project(core_id).unwrap();
        let dependent = workspace.get_project(dependent_id).unwrap();

        assert!(core.affected);
        assert!(dependent.affected);
    }
}
