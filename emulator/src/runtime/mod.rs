//! Runtime execution engine

/// App runtime engine
pub struct RuntimeEngine;

impl RuntimeEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn load_app(&self, _app_path: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn run_app(&self) -> Result<(), String> {
        Ok(())
    }
}

impl Default for RuntimeEngine {
    fn default() -> Self {
        Self::new()
    }
}
