## 環境構築
- gcc
- rustコンパイラ
- docker
- docker-compose
- postgresql
- openssl
    
```
# rustをインストール
$ curl https://sh.rustup.rs -sSf | sh
# (bashrcをいじるなりしてPATHを通す)
$ rustup update

# データベースを立てる
$ docker-compose up -d

# テーブル作成
$ psql -h localhost -p 5432 -U postgres -d goyotashi
# （sql/に従ってテーブルを作成）

# ビルド&run
$ cargo run
```

## Seeds
```
 INSERT INTO users (uid, name, email, icon) values ('3829-8392-jdij-3728', 'Taro', 'taro@api.goyotashi.com', 'http://icon.goyotashic.com');
 INSERT INTO communities (name, description, public ) values ('パレット', '生協です', true);
 INSERT INTO restaurants (vendor, place_id, name, addr, lat, lng) values (1, '36u16728', 'しばいけしょっぷ', '芝池街', 35.317971, 135.53157);
 INSERT INTO members (user_id, community_id) values (1,1);
 INSERT INTO pins (restaurant_id, community_id) values (1,1);
 INSERT INTO reviews (pin_id, member_id, comment) values (1,1, '良かった');
```

## reset
```
DROP SCHEMA IF EXISTS public CASCADE;
CREATE SCHEMA public;
```

## その他（centos）
- dependency
```bash
$ sudo yum install docker
```

- docker-compose
```
$ sudo curl -L "https://github.com/docker/compose/releases/download/1.25.4/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
$ sudo chmod +x /usr/local/bin/docker-compose
$ sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
```

## デプロイ
```
$ docker-compose -f release.docker-compose.yml up --build -d
$ docker exec -it goyotashi-database psql -h localhost -p 5432 -U postgres -d goyotashi
(テーブル作成)
```