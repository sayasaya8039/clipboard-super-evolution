//! Clipboard monitoring module

use arboard::Clipboard;
use std::thread;
use std::time::Duration;
use crate::analyzer::{ContentAnalyzer, ContentType, SuggestedAction};

pub struct ClipboardMonitor {
    clipboard: Clipboard,
    last_content: String,
    analyzer: ContentAnalyzer,
    history: Vec<ClipboardEntry>,
}

#[derive(Clone, Debug)]
pub struct ClipboardEntry {
    pub content: String,
    pub content_type: ContentType,
    pub timestamp: std::time::SystemTime,
    pub suggested_actions: Vec<SuggestedAction>,
}

impl ClipboardMonitor {
    pub fn new(analyzer: ContentAnalyzer) -> Result<Self, arboard::Error> {
        let clipboard = Clipboard::new()?;
        Ok(Self {
            clipboard,
            last_content: String::new(),
            analyzer,
            history: Vec::new(),
        })
    }

    pub fn start_monitoring(&mut self) -> Result<(), arboard::Error> {
        // Get initial content
        if let Ok(text) = self.clipboard.get_text() {
            self.last_content = text;
        }

        loop {
            match self.clipboard.get_text() {
                Ok(current_text) => {
                    if !current_text.is_empty() && current_text != self.last_content {
                        self.handle_new_content(&current_text);
                        self.last_content = current_text;
                    }
                }
                Err(arboard::Error::ContentNotAvailable) => {
                    // Non-text content (image, etc.) - skip
                }
                Err(e) => {
                    eprintln!("Clipboard read error: {}", e);
                }
            }

            thread::sleep(Duration::from_millis(300));
        }
    }

    fn handle_new_content(&mut self, content: &str) {
        println!("");
        println!("==================================================");
        println!("📋 New clipboard content detected!");
        println!("==================================================");

        // Analyze content
        let content_type = self.analyzer.analyze(content);
        let actions = self.analyzer.suggest_actions(&content_type, content);

        // Display content type
        println!("📊 Content Type: {:?}", content_type);

        let preview_len = content.len().min(100);
        println!("📝 Content Preview: {}", &content[..preview_len]);

        // Display suggested actions
        if !actions.is_empty() {
            println!("");
            println!("💡 Suggested Actions:");
            for (i, action) in actions.iter().enumerate() {
                println!("  {}. {} - {}", i + 1, action.icon, action.label);
            }
        }

        // Save to history
        let entry = ClipboardEntry {
            content: content.to_string(),
            content_type,
            timestamp: std::time::SystemTime::now(),
            suggested_actions: actions,
        };
        self.history.push(entry);

        println!("");
        println!("📚 History count: {}", self.history.len());
    }

    #[allow(dead_code)]
    pub fn get_history(&self) -> &[ClipboardEntry] {
        &self.history
    }
}
