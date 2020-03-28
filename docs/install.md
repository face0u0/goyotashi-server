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

## Seeds
```
 INSERT INTO users (uid) values ('3829-8392-jdij-3728');
 INSERT INTO communities (name, description, public ) values ('パレット', '生協です', true);
 INSERT INTO restaurants (place_id, name, lat, lng) values ('36u16728', 'しばいけしょっぷ', 35.317971, 135.53157);
 INSERT INTO members (user_id, community_id) values (1,1);
 INSERT INTO pins (restaurant_id, community_id) values (1,1);
 INSERT INTO reviews (pin_id, member_id, rate, comment) values (1,1, 3, '良かった');
```
