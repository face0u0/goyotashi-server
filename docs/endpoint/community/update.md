# コミュニティ更新 `[PUT] /communities/{id}`
`AUTH`

## Request
```json
{
	"name":  "やんパオ",
	"description":  "弁当製造業者です",
	"public": true
}
```

## Response

### 200 OK
```json
{
    "id": 1,
	"name":  "やんパオ",
	"description":  "弁当製造業者です",
	"public": true
}
```