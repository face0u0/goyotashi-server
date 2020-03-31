# コミュニティ追加 `[POST] /communities`
`AUTH`

## Request

```json
{
    "description":  "ギターをするサークル",
    "name":  "ギター部",
    "public": true
}
```

## Response 

### 200 Ok

- 作成したコミュニティには同時に参加します。

```json
{
    "id": 1,    
    "description":  "ギターをするサークル",
    "name":  "ギター部",
    "public": true
}
```