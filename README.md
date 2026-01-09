# 📋 Clipboard Super Evolution

> AIパワードのクリップボードマネージャー for Windows

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-0078d4.svg)](https://www.microsoft.com/windows)

## 概要

Clipboard Super Evolutionは、クリップボードの内容をAIが自動解析し、最適なアクションを提案するWindows向けクリップボードマネージャーです。

コピーした内容に応じて、翻訳・地図表示・コード改善提案などのアクションボタンがワンクリックで表示されます。

## ✨ 特徴

| 機能 | 説明 |
|------|------|
| 🔍 **スマート検出** | URL、メール、電話番号、住所、コードを自動識別 |
| 🌐 **URL** | ブラウザでワンクリック表示 |
| 🗺️ **住所** | Google Mapsで即座に表示 |
| 🔤 **英語テキスト** | Google翻訳で日本語に変換 |
| 💻 **コード** | 言語自動判定 (Rust/JS/Python/C++) |
| 📧 **メールアドレス** | メーラー起動 |
| 📞 **電話番号** | 通話アプリ起動 |
| 📚 **履歴管理** | 最大100件の履歴保存 |
| 🔎 **検索機能** | 履歴内を高速検索 |

## 📸 スクリーンショット

```
┌─────────────────────────────────────────┐
│ 📋 Clipboard Super Evolution        ⚙  │
├─────────────────────────────────────────┤
│ 🔍 [Search history...              ]    │
├─────────────────────────────────────────┤
│ ┌─────────────────────────────────────┐ │
│ │ 🌐 Url                    12:34:56  │ │
│ │ https://github.com/example/repo     │ │
│ │ [🌐 Open in Browser] [📋 Copy]      │ │
│ └─────────────────────────────────────┘ │
│ ┌─────────────────────────────────────┐ │
│ │ 🗺️ Address                12:30:22  │ │
│ │ 東京都渋谷区...                     │ │
│ │ [🗺️ Open in Google Maps] [📋 Copy] │ │
│ └─────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│ 📚 2 items                       v0.1.0│
└─────────────────────────────────────────┘
```

## 🚀 インストール

### ビルド済みバイナリ

```bash
# Releasesからダウンロード
clipboard_super_evolution.exe
```

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/your-username/clipboard-super-evolution.git
cd clipboard-super-evolution

# リリースビルド
cargo build --release

# 実行ファイルは以下に生成
./target/release/clipboard_super_evolution.exe
```

## 📦 依存関係

| クレート | バージョン | 用途 |
|----------|-----------|------|
| eframe | 0.30 | GUIフレームワーク |
| egui | 0.30 | 即時モードGUI |
| arboard | 3.4 | クロスプラットフォームクリップボード |
| regex | 1.11 | パターンマッチング |
| tokio | 1.41 | 非同期ランタイム |
| open | 5.3 | デフォルトブラウザ起動 |
| tray-icon | 0.19 | システムトレイ |
| global-hotkey | 0.6 | グローバルホットキー |

## 🏗️ アーキテクチャ

```
src/
├── main.rs        # egui GUIアプリケーション
│                  # - ウィンドウ管理
│                  # - クリップボード監視スレッド
│                  # - UI描画ループ
│
├── analyzer.rs    # コンテンツ解析エンジン
│                  # - 正規表現パターンマッチング
│                  # - コンテンツタイプ判定
│                  # - アクション提案ロジック
│
├── clipboard.rs   # クリップボード監視モジュール
│                  # - arboard連携
│                  # - 履歴管理
│
└── actions.rs     # アクション実行ユーティリティ
                   # - URL起動
                   # - 翻訳リンク生成
                   # - 地図リンク生成
```

## 🔧 コンテンツタイプ

```rust
pub enum ContentType {
    Url,           // https://... 形式
    Email,         // user@example.com
    Phone,         // 03-1234-5678, 09012345678
    Address,       // 東京都渋谷区...
    Code(String),  // fn, function, def, class...
    English,       // 英語テキスト (70%以上ASCII)
    Japanese,      // 日本語テキスト (ひらがな/カタカナ/漢字)
    Mixed,         // 日英混在
    Unknown,       // 不明
}
```

## ⌨️ 使い方

1. **アプリを起動**
   ```bash
   ./clipboard_super_evolution.exe
   ```

2. **何かをコピー** - 自動的に履歴に追加され、コンテンツタイプが解析されます

3. **アクションボタンをクリック** - 提案されたアクションを実行

4. **検索** - 上部の検索バーで履歴を検索

## 🛠️ 開発

```bash
# 開発ビルド（高速）
cargo build

# リリースビルド（最適化）
cargo build --release

# 実行
cargo run --release

# テスト
cargo test
```

## 📝 今後の予定

- [ ] システムトレイ常駐
- [ ] Ctrl+Shift+V ホットキーでポップアップ表示
- [ ] AI API連携（OpenAI/Anthropic/Gemini）
- [ ] コード改善提案機能
- [ ] 画像クリップボード対応
- [ ] クラウド同期

## 📄 ライセンス

MIT License

## 🙏 謝辞

- [egui](https://github.com/emilk/egui) - 素晴らしい即時モードGUIライブラリ
- [arboard](https://github.com/1Password/arboard) - クロスプラットフォームクリップボードライブラリ

---

Made with ❤️ and 🦀 Rust
