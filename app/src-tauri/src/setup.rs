use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn audio2sheets_dir() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".audio2sheets")
}

pub fn get_venv_python(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(resources) = app.path().resource_dir() {
        let bundled = resources.join("runtime").join("python.exe");
        if bundled.exists() {
            return Ok(bundled);
        }
    }
    let venv = audio2sheets_dir().join("venv");
    let installed = if cfg!(target_os = "windows") {
        venv.join("Scripts").join("python.exe")
    } else {
        venv.join("bin").join("python")
    };
    if installed.exists() {
        Ok(installed)
    } else {
        Err("Python runtime not found in the application resources.".to_string())
    }
}

pub fn get_python_dir(app: &AppHandle) -> Result<PathBuf, String> {
    // The development tree is not present on an installed user's machine.
    // In a packaged build, Tauri places the bundled Python sources in its
    // resource directory, which differs between Windows, macOS and Linux.
    if !cfg!(debug_assertions) {
        return app
            .path()
            .resource_dir()
            .map(|path| path.join("python"))
            .map_err(|error| format!("Could not locate application resources: {error}"));
    }
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("python");
    if dev_path.exists() {
        return Ok(dev_path);
    }
    app.path()
        .resource_dir()
        .map(|path| path.join("python"))
        .map_err(|error| format!("Could not locate application resources: {error}"))
}
