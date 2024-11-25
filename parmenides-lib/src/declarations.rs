use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::errors::BuildWorkspaceError;
use crate::project::{Project, ProjectId};
use crate::workspace::Workspace;

/// Represents a declaration of a project that can be used with `serde` for serialization and
/// deserialization
#[derive(Serialize, Deserialize)]
pub struct ProjectDeclaration {
    name: String,
    dependencies: Option<Vec<PathBuf>>,
}

/// Represents a declaration of a workspace that can be used with `serde` for serialization and
/// deserialization
#[derive(Serialize, Deserialize)]
pub struct WorkspaceDeclaration {
    projects: HashMap<PathBuf, ProjectDeclaration>,
}

impl WorkspaceDeclaration {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    pub fn add_project<P, S>(&mut self, path: P, name: S, dependencies: Option<Vec<PathBuf>>)
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        self.projects.insert(
            path.into(),
            ProjectDeclaration {
                name: name.into(),
                dependencies,
            },
        );
    }

    pub fn build_workspace(self) -> Result<Workspace, BuildWorkspaceError> {
        let mut workspace = Workspace::new();

        for path in self.projects.keys() {
            let mut stack = Vec::new();

            self.add_project_to_workspace(path, &mut workspace, &mut stack)?;
        }

        Ok(workspace)
    }

    fn add_project_to_workspace(
        &self,
        path: &PathBuf,
        workspace: &mut Workspace,
        stack: &mut Vec<PathBuf>,
    ) -> Result<ProjectId, BuildWorkspaceError> {
        if let Some(id) = workspace.get_id_by_path(path) {
            return Ok(id);
        }

        if stack.contains(path) {
            stack.push(path.clone());

            return Err(BuildWorkspaceError::CyclicDependencyFound(stack.clone()));
        }

        stack.push(path.clone());

        let declaration =
            self.projects
                .get(path)
                .ok_or(BuildWorkspaceError::ProjectDeclarationNotFound(
                    path.clone(),
                ))?;

        let dependencies = if let Some(dependencies) = &declaration.dependencies {
            let mut ids = Vec::with_capacity(dependencies.len());

            for dep in dependencies {
                let id = self.add_project_to_workspace(dep, workspace, stack)?;

                ids.push(id)
            }

            Some(ids)
        } else {
            None
        };

        let project = Project::new(path.clone(), declaration.name.clone(), dependencies);

        let id = workspace
            .add_project(project)
            .map_err(|err| BuildWorkspaceError::ErrorWhileAddingProject(path.clone(), err))?;

        Ok(id)
    }
}

impl Default for WorkspaceDeclaration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::errors::BuildWorkspaceError;

    use super::WorkspaceDeclaration;

    #[test]
    pub fn when_creating_from_declaration_should_build_workspace() {
        let mut workspace_declaration = WorkspaceDeclaration::new();

        let core_path = Path::new("/home/test/project/core").to_path_buf();
        let dependent_path = Path::new("/home/test/project/dependent").to_path_buf();

        workspace_declaration.add_project(core_path.clone(), "core", None);
        workspace_declaration.add_project(
            dependent_path.clone(),
            "dependent",
            Some(vec![core_path.clone()]),
        );

        let result = workspace_declaration.build_workspace();
        assert!(result.is_ok());

        let workspace = result.unwrap();
        assert_eq!(workspace.len(), 2);

        let core_project = workspace.get_project_by_path(&core_path).unwrap();
        assert_eq!(core_project.name, "core");

        let dependent_project = workspace.get_project_by_path(&dependent_path).unwrap();
        assert_eq!(dependent_project.name, "dependent");
    }

    #[test]
    pub fn when_creating_with_cyclic_dependency_should_return_error() {
        let mut workspace_declaration = WorkspaceDeclaration::new();

        let core_path = Path::new("/home/test/project/core").to_path_buf();
        let dependent_path = Path::new("/home/test/project/dependent").to_path_buf();

        workspace_declaration.add_project(
            core_path.clone(),
            "core",
            Some(vec![dependent_path.clone()]),
        );

        workspace_declaration.add_project(
            dependent_path.clone(),
            "dependent",
            Some(vec![core_path.clone()]),
        );

        let result = workspace_declaration.build_workspace();
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error,
            BuildWorkspaceError::CyclicDependencyFound(vec![
                core_path.clone(),
                dependent_path,
                core_path
            ])
        );
    }
}
