// Prevent console window on Windows
#![windows_subsystem = "windows"]

//! Clipboard Super Evolution - AI-powered clipboard manager
//! Version: 0.1.0

mod clipboard;
mod analyzer;
mod actions;
mod icon;

use eframe::egui::{self, FontData, FontDefinitions, FontFamily};
use std::sync::{Arc, Mutex};
use std::thread;
use arboard::Clipboard;
use crate::analyzer::{ContentAnalyzer, ContentType, SuggestedAction};

fn main() -> eframe::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🚀 Clipboard Super Evolution v0.1.0");

    // Generate icon
    let icon_image = icon::generate_icon_64();
    let icon_data = icon::to_icon_data(&icon_image);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_min_inner_size([300.0, 400.0])
            .with_title("Clipboard Super Evolution v0.1.0")
            .with_icon(std::sync::Arc::new(icon_data)),
        ..Default::default()
    };

    eframe::run_native(
        "Clipboard Super Evolution",
        options,
        Box::new(|cc| {
            // Configure Japanese fonts
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(ClipboardApp::new(cc)))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Try to load Windows Japanese fonts
    let font_paths = [
        "C:/Windows/Fonts/meiryo.ttc",      // Meiryo
        "C:/Windows/Fonts/msgothic.ttc",    // MS Gothic
        "C:/Windows/Fonts/YuGothM.ttc",     // Yu Gothic
        "C:/Windows/Fonts/msmincho.ttc",    // MS Mincho
    ];

    let mut font_loaded = false;

    for font_path in &font_paths {
        if let Ok(font_data) = std::fs::read(font_path) {
            fonts.font_data.insert(
                "japanese_font".to_owned(),
                FontData::from_owned(font_data).into(),
            );

            // Add Japanese font to proportional family (primary)
            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "japanese_font".to_owned());

            // Also add to monospace for code
            fonts
                .families
                .entry(FontFamily::Monospace)
                .or_default()
                .insert(0, "japanese_font".to_owned());

            println!("✅ Loaded Japanese font: {}", font_path);
            font_loaded = true;
            break;
        }
    }

    if !font_loaded {
        println!("⚠️ No Japanese font found, using default");
    }

    ctx.set_fonts(fonts);
}

struct ClipboardEntry {
    content: String,
    content_type: ContentType,
    actions: Vec<SuggestedAction>,
    timestamp: String,
}

struct ClipboardApp {
    history: Arc<Mutex<Vec<ClipboardEntry>>>,
    search_query: String,
    show_settings: bool,
}

impl ClipboardApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let history = Arc::new(Mutex::new(Vec::new()));
        let history_clone = history.clone();

        // Start clipboard monitoring in background thread
        thread::spawn(move || {
            let analyzer = ContentAnalyzer::new();
            let mut clipboard = match Clipboard::new() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to access clipboard: {}", e);
                    return;
                }
            };
            let mut last_content = String::new();

            loop {
                if let Ok(text) = clipboard.get_text() {
                    if !text.is_empty() && text != last_content {
                        let content_type = analyzer.analyze(&text);
                        let actions = analyzer.suggest_actions(&content_type, &text);

                        let entry = ClipboardEntry {
                            content: text.clone(),
                            content_type,
                            actions,
                            timestamp: chrono_now(),
                        };

                        if let Ok(mut h) = history_clone.lock() {
                            h.insert(0, entry);
                            // Keep only last 100 entries
                            if h.len() > 100 {
                                h.pop();
                            }
                        }

                        last_content = text;
                    }
                }
                thread::sleep(std::time::Duration::from_millis(300));
            }
        });

        Self {
            history,
            search_query: String::new(),
            show_settings: false,
        }
    }
}

fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    // Adjust for JST (UTC+9)
    let jst_secs = secs + 9 * 3600;
    let hours = (jst_secs / 3600) % 24;
    let mins = (jst_secs / 60) % 60;
    let s = jst_secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, s)
}

impl eframe::App for ClipboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint periodically to update UI
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("📋 Clipboard Super Evolution");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.show_settings = !self.show_settings;
                    }
                });
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add(egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("履歴を検索..."));
            });
        });

        if self.show_settings {
            egui::Window::new("設定")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("⚙️ 設定");
                    ui.separator();
                    ui.label("履歴上限: 100件");
                    ui.label("監視間隔: 300ms");
                    if ui.button("閉じる").clicked() {
                        self.show_settings = false;
                    }
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let history = self.history.lock().unwrap();
                let filtered: Vec<_> = history.iter()
                    .filter(|e| {
                        self.search_query.is_empty() ||
                        e.content.to_lowercase().contains(&self.search_query.to_lowercase())
                    })
                    .collect();

                if filtered.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label("📋 クリップボード履歴がありません");
                        ui.label("何かをコピーしてください！");
                    });
                } else {
                    for entry in filtered {
                        ui.group(|ui| {
                            // Header with type and timestamp
                            ui.horizontal(|ui| {
                                let (type_icon, type_label) = match &entry.content_type {
                                    ContentType::Url => ("🌐", "URL"),
                                    ContentType::Email => ("📧", "メール"),
                                    ContentType::Phone => ("📞", "電話番号"),
                                    ContentType::Address => ("🗺️", "住所"),
                                    ContentType::Code(lang) => ("💻", lang.as_str()),
                                    ContentType::English => ("🔤", "英語"),
                                    ContentType::Japanese => ("🇯🇵", "日本語"),
                                    ContentType::Mixed => ("🌏", "混合"),
                                    ContentType::Unknown => ("📝", "テキスト"),
                                };
                                ui.label(type_icon);
                                ui.label(type_label);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.small(&entry.timestamp);
                                });
                            });

                            // Content preview
                            let preview = if entry.content.chars().count() > 100 {
                                format!("{}...", entry.content.chars().take(100).collect::<String>())
                            } else {
                                entry.content.clone()
                            };
                            ui.label(&preview);

                            // Action buttons
                            ui.horizontal(|ui| {
                                for action in &entry.actions {
                                    let label = match action.label.as_str() {
                                        "Open in Browser" => "ブラウザで開く",
                                        "Open in Google Maps" => "マップで開く",
                                        "Translate to Japanese" => "翻訳",
                                        "Compose email" => "メール作成",
                                        "Call this number" => "電話する",
                                        "Save to favorites" => "お気に入り",
                                        _ => &action.label,
                                    };
                                    if ui.small_button(format!("{} {}", action.icon, label)).clicked() {
                                        if let Some(url) = &action.url {
                                            let _ = open::that(url);
                                        }
                                    }
                                }
                                if ui.small_button("📋 コピー").clicked() {
                                    if let Ok(mut clipboard) = Clipboard::new() {
                                        let _ = clipboard.set_text(&entry.content);
                                    }
                                }
                            });
                        });
                        ui.add_space(5.0);
                    }
                }
            });
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let history_len = self.history.lock().map(|h| h.len()).unwrap_or(0);
                ui.label(format!("📚 {} 件", history_len));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.small("v0.1.0");
                });
            });
        });
    }
}
