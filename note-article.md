# 【Rust】AIパワードクリップボードマネージャー「Clipboard Super Evolution」を作った

## はじめに

コピペ作業、毎日何回やっていますか？

URLをコピーしてブラウザに貼り付け、住所をコピーしてGoogle Mapsで検索、英語テキストをコピーして翻訳サイトへ...

**「コピーした瞬間に最適なアクションが提案されたら便利じゃない？」**

そう思って作りました。

## Clipboard Super Evolutionとは

クリップボードの内容をリアルタイムで解析し、コンテンツタイプに応じた最適なアクションをワンクリックで実行できるWindowsアプリケーションです。

### 主な機能

📋 **スマート検出**
- URL → 「ブラウザで開く」ボタン
- 住所（日本語） → 「Google Mapsで開く」ボタン
- 英語テキスト → 「日本語に翻訳」ボタン
- コード → 言語自動判定（Rust/JS/Python/C++）
- メールアドレス → メーラー起動
- 電話番号 → 通話アプリ起動

📚 **履歴管理**
- 最大100件の履歴保存
- 高速検索機能
- タイムスタンプ表示

## 技術スタック

Rustで開発しました。主な依存関係：

| ライブラリ | 用途 |
|-----------|------|
| **egui/eframe** | GUIフレームワーク |
| **arboard** | クリップボード操作 |
| **regex** | パターンマッチング |
| **tokio** | 非同期処理 |

### なぜRust？

1. **高速**: ネイティブバイナリで起動が爆速
2. **省メモリ**: 常駐させても負担が少ない
3. **クロスプラットフォーム**: 将来的にmacOS/Linux対応も視野に
4. **型安全**: バグの少ないコードが書ける

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                               │
│  ┌─────────────────┐    ┌─────────────────────────────────┐ │
│  │  egui Window    │    │  Background Thread              │ │
│  │  - Header       │    │  - Clipboard Monitor (300ms)    │ │
│  │  - Search       │◄───│  - Content Analysis             │ │
│  │  - History List │    │  - History Update               │ │
│  │  - Footer       │    │                                 │ │
│  └─────────────────┘    └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
      ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
      │ analyzer.rs │ │clipboard.rs │ │ actions.rs  │
      │ - Regex     │ │ - arboard   │ │ - open URL  │
      │ - Detect    │ │ - History   │ │ - Maps link │
      │ - Suggest   │ │             │ │ - Translate │
      └─────────────┘ └─────────────┘ └─────────────┘
```

## コンテンツ検出ロジック

正規表現でパターンマッチングしています：

```rust
pub struct ContentAnalyzer {
    url_pattern: Regex,      // https?://...
    email_pattern: Regex,    // user@domain.com
    phone_pattern: Regex,    // 03-1234-5678
    address_pattern: Regex,  // 東京都...市区町村
    code_pattern: Regex,     // fn, function, def, class...
    japanese_pattern: Regex, // ひらがな/カタカナ/漢字
}
```

言語判定も実装：

```rust
fn detect_language(&self, content: &str) -> String {
    if content.contains("fn ") || content.contains("pub ") {
        "Rust".to_string()
    } else if content.contains("function") || content.contains("=>") {
        "JavaScript".to_string()
    } else if content.contains("def ") {
        "Python".to_string()
    } else if content.contains("#include") {
        "C/C++".to_string()
    } else {
        "Unknown".to_string()
    }
}
```

## バイナリサイズ

リリースビルドで **約7.6MB**。

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

LTO（Link Time Optimization）とstrip有効化で最適化しています。

## 今後の展望

### 近日実装予定
- 🔔 システムトレイ常駐
- ⌨️ グローバルホットキー（Ctrl+Shift+V）
- 🤖 AI API連携（GPT-4/Claude/Gemini）

### 将来的に
- 📸 画像クリップボード対応（OCR）
- ☁️ クラウド同期
- 🍎 macOS/Linux対応

## まとめ

Rustでクリップボードマネージャーを作りました。

egui/eframeは学習コストが低く、Rust初心者でもGUIアプリが作りやすいです。arboardはクロスプラットフォーム対応で、Windows/macOS/Linuxで同じコードが動きます。

コピペ作業を効率化したい方、ぜひ試してみてください！

---

**GitHub**: （リポジトリURL）

**ハッシュタグ**: #Rust #egui #Windows #クリップボード #開発 #プログラミング
