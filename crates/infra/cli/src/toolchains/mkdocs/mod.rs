use std::path::Path;

use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

pub struct Mkdocs;

impl Mkdocs {
    pub fn build() -> Result<()> {
        mkdocs_command()
            .arg("build")
            .flag("--clean")
            .flag("--strict")
            .run()
    }

    pub fn watch() -> Result<()> {
        // _MKDOCS_WATCH_PORT_ | keep in sync with the port number defined in "$REPO_ROOT/.devcontainer/devcontainer.json"
        const PORT: usize = 5353;

        mkdocs_command()
            .arg("serve")
            .flag("--clean")
            .flag("--watch-theme")
            .property("--dev-addr", format!("localhost:{PORT}"))
            .run()
    }
}

fn mkdocs_command() -> Command {
    Command::new("python3")
        .property("-m", "pipenv")
        .args(["run", "mkdocs"])
        .current_dir(Path::repo_path("documentation"))
}
