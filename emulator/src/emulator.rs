//! Main emulator controller

use crate::{Device, Result, iOSVersion, Error};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main iOSS Emulator controller
pub struct Emulator {
    devices: HashMap<String, Device>,
    home_dir: PathBuf,
}

impl Emulator {
    /// Create a new emulator instance
    pub fn new(home_dir: Option<PathBuf>) -> Result<Self> {
        let home_dir = home_dir.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".ioss")
        });

        Ok(Self {
            devices: HashMap::new(),
            home_dir,
        })
    }

    /// Create a new virtual device
    pub fn create_device(
        &mut self,
        name: String,
        ios_version: iOSVersion,
    ) -> Result<String> {
        let device_id = uuid::Uuid::new_v4().to_string();
        let device_dir = self.home_dir.join("devices").join(&device_id);

        let device = Device::new(
            device_id.clone(),
            name,
            ios_version,
            device_dir,
        );

        self.devices.insert(device_id.clone(), device);
        Ok(device_id)
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: &str) -> Result<&Device> {
        self.devices
            .get(device_id)
            .ok_or_else(|| Error::NotFound(format!("Device not found: {}", device_id)))
    }

    /// Get a mutable device by ID
    pub fn get_device_mut(&mut self, device_id: &str) -> Result<&mut Device> {
        self.devices
            .get_mut(device_id)
            .ok_or_else(|| Error::NotFound(format!("Device not found: {}", device_id)))
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    /// Delete a device
    pub fn delete_device(&mut self, device_id: &str) -> Result<()> {
        if let Some(device) = self.devices.remove(device_id) {
            // Clean up storage
            if device.storage_path.exists() {
                std::fs::remove_dir_all(&device.storage_path)?;
            }
            Ok(())
        } else {
            Err(Error::NotFound(format!("Device not found: {}", device_id)))
        }
    }

    /// Get emulator home directory
    pub fn home_dir(&self) -> &Path {
        &self.home_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_emulator_creation() {
        let temp_dir = TempDir::new().unwrap();
        let emulator = Emulator::new(Some(temp_dir.path().to_path_buf())).unwrap();
        assert_eq!(emulator.list_devices().len(), 0);
    }

    #[test]
    fn test_create_device() {
        let temp_dir = TempDir::new().unwrap();
        let mut emulator = Emulator::new(Some(temp_dir.path().to_path_buf())).unwrap();
        
        let device_id = emulator
            .create_device("Test Device".to_string(), iOSVersion::iOS15)
            .unwrap();

        assert_eq!(emulator.list_devices().len(), 1);
        assert!(emulator.get_device(&device_id).is_ok());
    }
}
