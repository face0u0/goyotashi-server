# （付近の）レストランを検索 `[GET] /restaurants`

## Request

|key|val|require|
|:--|:--|:--|
|lat|35.112673|o|
|lng|135.172921|o|
|name|"やんぱお"||

## Response

### 200 Ok
```json
[
    {
        "id": 3,
        "vendor": 1,
        "place_id": "7483189",
        "name": "ろぶた LOBUTA",
        "addr": "〒606-8212 京都府京都市左京区田中里ノ内町48 ",
        "lat": 35.033608,
        "lng": 135.781213
    }
]
```