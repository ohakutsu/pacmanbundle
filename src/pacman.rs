use anyhow::{Context, Result};
use std::{collections::HashSet, process::Command};

pub struct Pacman {}

impl Pacman {
    pub fn get_installed_packages() -> Result<Vec<String>> {
        let package_manager = "pacman";

        let mut cmd = Command::new(package_manager);
        let output = cmd
            .arg("--query")
            .arg("--explicit")
            .arg("--quiet")
            .arg("--native")
            .output()
            .with_context(|| format!("Failed to execute `{}`", package_manager))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let packages_uniq = stdout.trim().lines().collect::<HashSet<_>>();
        let mut packages: Vec<_> = packages_uniq.into_iter().map(|s| s.to_string()).collect();
        packages.sort();

        Ok(packages)
    }
}
