//! System services (display, input, notifications, etc.)

/// Display service
pub struct DisplayService;

impl DisplayService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DisplayService {
    fn default() -> Self {
        Self::new()
    }
}

/// Input service for touch and keyboard events
pub struct InputService;

impl InputService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InputService {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification service
pub struct NotificationService;

impl NotificationService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
