# learning-criterion.rs

🌤🌤🌤 Rustのベンチマーククレートを試してみる！  

## 実行方法

DevContainerに入り、以下のコマンドを実行します。  

```shell
# モジュールのインストール
cargo install --path .

# ベンチマークの実行  
# 実行結果は`./target/criterion/report/index.html`に出力されます。
cargo bench

# 開発用の実行
cargo run

# ビルド＆実行
cargo build --release && ./target/release/learning-criterion-rs
```
