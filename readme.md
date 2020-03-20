# Goyotashi Server

## 環境構築
```
# rustをインストール
$ curl https://sh.rustup.rs -sSf | sh
$ rustup update

# データベースを立てる
$ docker-compose up

# テーブル作成
$ psql -h localhost -p 5432 -U postgres -d goyotashi
# （sql/に従ってテーブルを作成）

# ビルド&run
$ cargo run
```

## seeds
```
 INSERT INTO users (uid) values ('3829-8392-jdij-3728');
 INSERT INTO communities (name, description, public ) values ('パレット', '生協です', true);
 INSERT INTO restaurants (place_id, name, lat, lng) values ('36u16728', 'しばいけしょっぷ', 35.317971, 135.53157);
 INSERT INTO members (user_id, community_id) values (1,1);
 INSERT INTO pins (restaurant_id, community_id) values (1,1);
 INSERT INTO reviews (pin_id, member_id, rate, comment) values (1,1, 3, '良かった');
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