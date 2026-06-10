use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PathResolver {
    home_dir: PathBuf,
    workspace_dir: Option<PathBuf>,
}

impl PathResolver {
    pub fn new(home_dir: PathBuf, workspace_dir: Option<PathBuf>) -> Self {
        Self {
            home_dir,
            workspace_dir,
        }
    }

    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }

    pub fn workspace_dir(&self) -> Option<&Path> {
        self.workspace_dir.as_deref()
    }

    pub fn home_path(&self, relative: &str) -> PathBuf {
        self.home_dir.join(relative)
    }

    pub fn workspace_path(&self, relative: &str) -> Option<PathBuf> {
        self.workspace_dir.as_ref().map(|dir| dir.join(relative))
    }
}
