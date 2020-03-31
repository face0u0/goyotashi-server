# Goyotashi Server Document

Goyotashiのサーバーサイドです。ドキュメントは

- https://face0u0.github.io/goyotashi-server
- https://api.goyotashi.com/docs

にあります。githubの方は最新のmasterに従っており、api.goyotashi.comの方はホスティングされているバージョンに従っています。適宜使い分けて下さい。

## 環境
- rust
- rocket(マイクロフレームワーク)
- PostgreSQL(nosqlはアダプタがまだ怪しいのでパス)
- docker