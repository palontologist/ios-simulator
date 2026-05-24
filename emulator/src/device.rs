//! Device simulation - represents a virtual iOS device

use crate::{Error, Result, iOSVersion};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Virtual iOS Device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Unique device identifier
    pub id: String,
    /// Device name (e.g., "iPhone 12")
    pub name: String,
    /// iOS version
    pub ios_version: String,
    /// Device state
    pub state: DeviceState,
    /// Device properties
    pub properties: DeviceProperties,
    /// Storage path
    #[serde(skip)]
    pub storage_path: PathBuf,
}

/// Device state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceState {
    /// Device created but not booted
    Shut Down,
    /// Device is booting
    Booting,
    /// Device is running
    Running,
    /// Device is shutting down
    Shutting Down,
    /// Device encountered an error
    Error,
}

impl std::fmt::Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceState::Shut Down => write!(f, "Shut Down"),
            DeviceState::Booting => write!(f, "Booting"),
            DeviceState::Running => write!(f, "Running"),
            DeviceState::Shutting Down => write!(f, "Shutting Down"),
            DeviceState::Error => write!(f, "Error"),
        }
    }
}

/// Device hardware properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProperties {
    /// Architecture (e.g., "arm64")
    pub architecture: String,
    /// Screen resolution width
    pub screen_width: u32,
    /// Screen resolution height
    pub screen_height: u32,
    /// Available RAM in MB
    pub ram_mb: u32,
    /// Available storage in MB
    pub storage_mb: u32,
    /// Device model identifier (e.g., "A2847" for iPhone 12)
    pub model_identifier: String,
    /// System name
    pub system_name: String,
}

impl Default for DeviceProperties {
    fn default() -> Self {
        // iPhone 12 equivalent
        Self {
            architecture: "arm64".to_string(),
            screen_width: 390,
            screen_height: 844,
            ram_mb: 4096,
            storage_mb: 128000,
            model_identifier: "A2847".to_string(),
            system_name: "iOSS".to_string(),
        }
    }
}

impl Device {
    /// Create a new device
    pub fn new(
        id: String,
        name: String,
        ios_version: iOSVersion,
        storage_path: PathBuf,
    ) -> Self {
        Self {
            id,
            name,
            ios_version: ios_version.as_str().to_string(),
            state: DeviceState::Shut Down,
            properties: DeviceProperties::default(),
            storage_path,
        }
    }

    /// Boot the device
    pub async fn boot(&mut self) -> Result<()> {
        if self.state == DeviceState::Running {
            return Err(Error::InvalidState("Device is already running".to_string()));
        }

        self.state = DeviceState::Booting;
        
        // Initialize device filesystem
        self.initialize_filesystem().await?;

        self.state = DeviceState::Running;
        Ok(())
    }

    /// Shutdown the device
    pub async fn shutdown(&mut self) -> Result<()> {
        if self.state == DeviceState::Shut Down {
            return Ok(());
        }

        self.state = DeviceState::Shutting Down;
        
        // Cleanup resources
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        self.state = DeviceState::Shut Down;
        Ok(())
    }

    /// Initialize virtual filesystem
    async fn initialize_filesystem(&self) -> Result<()> {
        let paths = vec![
            "apps",
            "filesystem/Documents",
            "filesystem/Downloads",
            "filesystem/Library/Caches",
            "filesystem/Library/Preferences",
            "filesystem/Library/Application Support",
            "filesystem/tmp",
            "filesystem/var/log",
        ];

        for path in paths {
            let full_path = self.storage_path.join(path);
            tokio::fs::create_dir_all(&full_path)
                .await
                .map_err(|e| Error::Device(format!("Failed to create directory {}: {}", path, e)))?;
        }

        // Create settings file
        let settings = serde_json::json!({
            "device_id": self.id,
            "device_name": self.name,
            "ios_version": self.ios_version,
            "created_at": chrono::Local::now().to_rfc3339(),
        });

        let settings_path = self.storage_path.join("settings.json");
        tokio::fs::write(&settings_path, serde_json::to_string_pretty(&settings)?)
            .await?;

        Ok(())
    }

    /// Get device filesystem root
    pub fn filesystem_root(&self) -> &Path {
        &self.storage_path
    }

    /// Install an app on the device
    pub async fn install_app(&self, app_path: &Path, app_id: &str) -> Result<()> {
        if self.state != DeviceState::Running {
            return Err(Error::InvalidState("Device is not running".to_string()));
        }

        let apps_dir = self.storage_path.join("apps");
        let app_dir = apps_dir.join(app_id);

        tokio::fs::create_dir_all(&app_dir).await?;

        // Copy app bundle
        if app_path.is_file() {
            // If it's an IPA, extract it
            let content = tokio::fs::read(app_path).await?;
            // For now, just copy the file
            tokio::fs::write(app_dir.join("app.bin"), content).await?;
        } else {
            // Copy directory contents
            for entry in walkdir::WalkDir::new(app_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let relative = entry.path().strip_prefix(app_path)?;
                let target = app_dir.join(relative);
                
                if entry.path().is_dir() {
                    tokio::fs::create_dir_all(&target).await?;
                } else {
                    tokio::fs::copy(entry.path(), &target).await?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_device_creation() {
        let temp_dir = TempDir::new().unwrap();
        let device = Device::new(
            "test-device".to_string(),
            "Test Device".to_string(),
            iOSVersion::iOS15,
            temp_dir.path().to_path_buf(),
        );

        assert_eq!(device.id, "test-device");
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.state, DeviceState::Shut Down);
    }

    #[tokio::test]
    async fn test_device_boot_shutdown() {
        let temp_dir = TempDir::new().unwrap();
        let mut device = Device::new(
            "test-device".to_string(),
            "Test Device".to_string(),
            iOSVersion::iOS15,
            temp_dir.path().to_path_buf(),
        );

        device.boot().await.unwrap();
        assert_eq!(device.state, DeviceState::Running);

        device.shutdown().await.unwrap();
        assert_eq!(device.state, DeviceState::Shut Down);
    }
}
