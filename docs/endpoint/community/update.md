# コミュニティアップデート `[PUT] /communities/{id}`
`AUTH`

## request
```json
{
	"name":  "やんパオ",
	"description":  "弁当製造業者です",
	"public": true
}
```

## response

### 200 OK
```json
{
    "id": 1,
	"name":  "やんパオ",
	"description":  "弁当製造業者です",
	"public": true
}
```