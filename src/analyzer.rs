//! Content analysis module - detects content type and suggests actions

use regex::Regex;

pub struct ContentAnalyzer {
    // Regex patterns for content detection
    url_pattern: Regex,
    email_pattern: Regex,
    phone_pattern: Regex,
    address_pattern: Regex,
    code_pattern: Regex,
    japanese_pattern: Regex,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContentType {
    Url,
    Email,
    Phone,
    Address,
    Code(String), // Language hint
    English,
    Japanese,
    Mixed,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct SuggestedAction {
    pub icon: &'static str,
    pub label: String,
    pub action_type: ActionType,
    pub url: Option<String>,
}

#[derive(Clone, Debug)]
pub enum ActionType {
    OpenInBrowser,
    OpenInMaps,
    Translate,
    SearchCode,
    SuggestFix,
    Copy,
    Save,
}

impl ContentAnalyzer {
    pub fn new() -> Self {
        Self {
            url_pattern: Regex::new(r"https?://[\w\-\.]+\.[a-zA-Z]{2,}[\S]*").unwrap(),
            email_pattern: Regex::new(r"[\w\.\-]+@[\w\-]+\.[a-zA-Z]{2,}").unwrap(),
            phone_pattern: Regex::new(r"(\d{2,4}[\-\s]?\d{2,4}[\-\s]?\d{3,4})|0\d{9,10}").unwrap(),
            address_pattern: Regex::new(r"(東京|大阪|京都|北海道|.{2,3}県).{2,}.{1,3}(市|区|町|村)").unwrap(),
            code_pattern: Regex::new(r"(fn |function |def |class |const |let |var |import |pub |async )").unwrap(),
            japanese_pattern: Regex::new(r"[\p{Hiragana}\p{Katakana}\p{Han}]").unwrap(),
        }
    }

    pub fn analyze(&self, content: &str) -> ContentType {
        let trimmed = content.trim();

        // Check URL first
        if self.url_pattern.is_match(trimmed) {
            return ContentType::Url;
        }

        // Check email
        if self.email_pattern.is_match(trimmed) {
            return ContentType::Email;
        }

        // Check phone
        if self.phone_pattern.is_match(trimmed) {
            return ContentType::Phone;
        }

        // Check address (Japanese)
        if self.address_pattern.is_match(trimmed) {
            return ContentType::Address;
        }

        // Check code
        if self.code_pattern.is_match(trimmed) {
            let lang = self.detect_language(trimmed);
            return ContentType::Code(lang);
        }

        // Check if English or Japanese text
        let has_japanese = self.japanese_pattern.is_match(trimmed);
        let is_mostly_english = self.is_mostly_english(trimmed);

        if !has_japanese && is_mostly_english {
            return ContentType::English;
        }

        if has_japanese && !is_mostly_english {
            return ContentType::Japanese;
        }

        if has_japanese && is_mostly_english {
            return ContentType::Mixed;
        }

        ContentType::Unknown
    }

    fn is_mostly_english(&self, content: &str) -> bool {
        let english_chars: usize = content.chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
            .count();
        let total_chars = content.chars().count();

        if total_chars == 0 { return false; }
        (english_chars as f64 / total_chars as f64) > 0.7
    }

    fn detect_language(&self, content: &str) -> String {
        if content.contains("fn ") || content.contains("let mut") || content.contains("pub ") {
            "Rust".to_string()
        } else if content.contains("function") || content.contains("const ") || content.contains("=>") {
            "JavaScript".to_string()
        } else if content.contains("def ") || (content.contains("import ") && !content.contains("{")) {
            "Python".to_string()
        } else if content.contains("#include") || content.contains("int main") {
            "C/C++".to_string()
        } else {
            "Unknown".to_string()
        }
    }

    pub fn suggest_actions(&self, content_type: &ContentType, content: &str) -> Vec<SuggestedAction> {
        let mut actions = Vec::new();

        match content_type {
            ContentType::Url => {
                actions.push(SuggestedAction {
                    icon: "🌐",
                    label: "Open in Browser".to_string(),
                    action_type: ActionType::OpenInBrowser,
                    url: Some(content.to_string()),
                });
            }
            ContentType::Address => {
                let maps_url = format!(
                    "https://www.google.com/maps/search/{}",
                    urlencoding_simple(content)
                );
                actions.push(SuggestedAction {
                    icon: "🗺️",
                    label: "Open in Google Maps".to_string(),
                    action_type: ActionType::OpenInMaps,
                    url: Some(maps_url),
                });
            }
            ContentType::English => {
                let translate_url = format!(
                    "https://translate.google.com/?sl=en&tl=ja&text={}",
                    urlencoding_simple(content)
                );
                actions.push(SuggestedAction {
                    icon: "🔤",
                    label: "Translate to Japanese".to_string(),
                    action_type: ActionType::Translate,
                    url: Some(translate_url),
                });
            }
            ContentType::Code(lang) => {
                actions.push(SuggestedAction {
                    icon: "🔧",
                    label: format!("Suggest {} code improvements", lang),
                    action_type: ActionType::SuggestFix,
                    url: None,
                });
                actions.push(SuggestedAction {
                    icon: "🔍",
                    label: format!("Search {} documentation", lang),
                    action_type: ActionType::SearchCode,
                    url: None,
                });
            }
            ContentType::Email => {
                actions.push(SuggestedAction {
                    icon: "📧",
                    label: "Compose email".to_string(),
                    action_type: ActionType::OpenInBrowser,
                    url: Some(format!("mailto:{}", content)),
                });
            }
            ContentType::Phone => {
                actions.push(SuggestedAction {
                    icon: "📞",
                    label: "Call this number".to_string(),
                    action_type: ActionType::OpenInBrowser,
                    url: Some(format!("tel:{}", content.replace("-", "").replace(" ", ""))),
                });
            }
            _ => {}
        }

        // Common action: Save to history
        actions.push(SuggestedAction {
            icon: "💾",
            label: "Save to favorites".to_string(),
            action_type: ActionType::Save,
            url: None,
        });

        actions
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
