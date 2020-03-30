# 認可(Authorization)について

- 当初の予定通り、認証についてはFirebase Authenticationを用います。
- goyotashi-serverではFirebaseから発行される、jwtをもちいて認証を行います。

`AUTH`と書かれているエンドポイントでは、以下のようにHeaderを設定して下さい。

|key|value|
|:--|:--|
|Authorization|Bearer {your jwt token}|

上記のように設定すると、サーバーサイドでjwtの検証を行います。

うまくHeaderが設定されていなかったり、不正な署名によるjwtであった場合は401Unauthorizedが返ります。

