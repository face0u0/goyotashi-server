# Goyotashi Server

## 環境構築
```
# rustをインストール
$ curl https://sh.rustup.rs -sSf | sh

# データベースを立てる
$ docker-compose up

# テーブル作成
$ psql -h localhost -p 5432 -U postgres -d goyotashi
# （sql/に従ってテーブルを作成）

# ビルド&run
$ cargo build
$ cargo run

```

## Schema
### community
- コミュニティを表す

### user
- ユーザーを表す
- firebaseのuidと一対一ね

### member
- ユーザーとコミュニティの中間テーブル
- コミュニティへの参加により生成される

### restaurant
- 店を表す
- googlemapの店と一対一

### pin
- コミュニティと店の中間テーブル
- 店を追加すると生成される

### review
- pinとmember/userの中間テーブル(どっちにするか迷ってる)

## 