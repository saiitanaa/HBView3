use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Project {
    pub root: PathBuf,
    pub sources: Vec<PathBuf>,
    pub headers: Vec<PathBuf>,
    pub assets: Vec<PathBuf>,
    pub makefile: Option<PathBuf>,
}

impl Project {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let root = path
            .as_ref()
            .canonicalize()
            .map_err(|e| format!("Impossible d'ouvrir le projet: {e}"))?;

        if !root.is_dir() {
            return Err(format!("{} n'est pas un dossier", root.display()));
        }

        let mut project = Self {
            root: root.clone(),
            sources: Vec::new(),
            headers: Vec::new(),
            assets: Vec::new(),
            makefile: None,
        };

        scan_directory(&root, &mut project)?;

        Ok(project)
    }

    pub fn name(&self) -> String {
        self.root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    }
}

fn scan_directory(path: &Path, project: &mut Project) -> Result<(), String> {
    for entry in
        fs::read_dir(path).map_err(|e| format!("Impossible de lire {}: {e}", path.display()))?
    {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, project)?;
            continue;
        }

        let Some(extension) = path.extension().and_then(|e| e.to_str()) else {
            if path.file_name().and_then(|n| n.to_str()) == Some("Makefile") {
                project.makefile = Some(path);
            }
            continue;
        };

        match extension {
            "c" | "cpp" | "cc" | "cxx" => {
                project.sources.push(path);
            }

            "h" | "hpp" | "hh" | "hxx" => {
                project.headers.push(path);
            }

            "png" | "jpg" | "jpeg" | "bmp" | "tga" | "wav" | "ogg" => {
                project.assets.push(path);
            }

            _ => {}
        }
    }

    Ok(())
}
