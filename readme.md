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

## Seeds
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

## Endpoint
### コミュニティの飯屋`[GET] /communities/{id}/pins`
- request 
```
 no body
```
- response (200 OK)
    - 成功時
```json
[
  {
    "id": 611,
    "restaurant_id": 181,
    "community_id": 116
  }
]
```
- response (404 Not Found)
    - idが無効
```json
{
  "msg": "resource not found."
}
```

### コミュニティ情報 `[GET] /communities/{id}`
- request
```
 no body
```
- response (200 OK)
    - 成功時
```json
{
  "id": 2819,
  "name": "パレット",
  "description": "生協のサークル",
  "public": true
}
```
- response (404 Not Found)
    - idが無効
```json
{
  "msg": "resource not found."
}
```
- response (401 Unauthorized)
    - private_communityにアクセスしたが、jwtがない
```json
{
  "msg": "this resource need token."
}
```
- response (403 Forbidden)
    - private_communityにアクセスしたが、アクセス権がない
```json
{
  "msg": "forbidden."
}
```

### コミュニティ検索 `[GET] /communities`
- request
```json
{
  "search": "パレット"
}
```
- response (200 OK)
    - 成功時
```json
[
    {
      "id": 2819,
      "name": "パレット",
      "description": "生協のサークル",
      "public": true
    } 
]
```
- response (400 Bad Request)
    - searchを指定してない
```json
{
  "msg": "no search query."
}
```


### コミュニティ追加 `[POST] /communities`
- request
```json
{
    "description":  "ギターをするサークル",
    "name":  "ギター部",
    "public": true
}
```
- response (200 OK)
    - 成功時
```
no body
```
- response (400 Bad Request)
    - requestが不正
```json
{
  "msg": "invalid"
}
```

### コミュニティアップデート `[PUT] /communities/{id}`
- request
    - 必要なものだけ入れればおけ
```json
{
	"name":  "やんパオ",
	"description":  "弁当製造業者です",
	"public": true
}
```
- response (200 OK)
    - 成功時
```
no body
```
- response (401 Unauthorized)
    - jwtがない
```json
{
  "msg": "need auth."
}
```
- response (403 Forbidden)
    - アクセス権がない（参加していない）
```json
{
  "msg": "forbidden."
}
```
### コミュニティ参加 `[POST] /members`
- request
```json
{
	"community_id": 12
}
```
- response (200 OK)
    - 成功時
```
no body
```
- response (401 Unauthorized)
    - jwtがない
```json
{
  "msg": "need auth."
}
```
### 店追加 `[POST] /pins`
- request
```json
{
	"place_id": 391021,
	"community_id": 19201
}
```
- response (200 OK)
```
no body
```
- response (404 Not Found)
```json
{
  "msg": "no community found"
}
```
- response (401 Unauthorized)
    - jwtがない
```json
{
  "msg": "need auth."
}
```
- response (403 Forbidden)
    - アクセス権がない（参加していない）
```json
{
  "msg": "forbidden."
}
```
### 店検索 `[GET] /gplace`
- google search apiのラッパー
```json
{
	"search": "やんパオ"
}
```
```
[
	{{restaurant_list}}
]

```

### 店情報 `[GET] /gplace/{id}`
- google detail apiのラッパー
```
no body
```
```
{{restaurant}}
```

