//! Action execution module

use std::process::Command;

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn new() -> Self {
        Self
    }

    /// Open URL in default browser
    pub fn open_url(&self, url: &str) -> Result<(), std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", "", url])
                .spawn()?;
        }

        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(url)
                .spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open")
                .arg(url)
                .spawn()?;
        }

        Ok(())
    }

    /// Open Google Maps with address
    pub fn open_maps(&self, address: &str) -> Result<(), std::io::Error> {
        let encoded = urlencoding_simple(address);
        let url = format!("https://www.google.com/maps/search/{}", encoded);
        self.open_url(&url)
    }

    /// Open Google Translate
    pub fn translate(&self, text: &str, from: &str, to: &str) -> Result<(), std::io::Error> {
        let encoded = urlencoding_simple(text);
        let url = format!(
            "https://translate.google.com/?sl={}&tl={}&text={}",
            from, to, encoded
        );
        self.open_url(&url)
    }
}

fn urlencoding_simple(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "%20".to_string(),
            '&' => "%26".to_string(),
            '?' => "%3F".to_string(),
            '=' => "%3D".to_string(),
            _ if c.is_ascii_alphanumeric() => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
