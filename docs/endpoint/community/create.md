# コミュニティ追加 `[POST] /communities`
`AUTH`

## request

```json
{
    "description":  "ギターをするサークル",
    "name":  "ギター部",
    "public": true
}
```

## response 

### Ok(200)

- 作成したコミュニティには同時に参加します。

```json
{
    "id": 1,    
    "description":  "ギターをするサークル",
    "name":  "ギター部",
    "public": true
}
```