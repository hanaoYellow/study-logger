# study-logger

コマンドラインで動く学習記録管理ツール．学習した科目・時間・日付をJSONで記録し，科目別の集計ができる．

## 使い方

### 記録を追加
```bash
cargo run -- add "Rust" 60
```

### 記録一覧を表示
```bash
cargo run -- list
```

### 科目別集計を表示
```bash
cargo run -- summary
```

## 出力例

### list
```
date        subject    minutes
--------------------------------
2026-03-15  Rust       60分
2026-03-15  競プロ     90分
2026-03-15  Rust       45分
```

### summary
```
科目別集計
----------------------
Rust       105分
競プロ     90分
```

## 使用クレート
- `serde` / `serde_json`: 学習記録のJSON読み書き
- `chrono`: 日付の取得・フォーマット

## 学んだこと
- enumでサブコマンドを管理
- serdeを使ったJSON読み書き
- &strとStringの使い分け
- HashMapによる集計と降順ソート
