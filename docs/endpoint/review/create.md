# いいねorレビュー追加 `[POST] /reviews`
`AUTH`

## Request
```json
{
  "pin_id": 1,
  "good": true, 
  "comment": "あんまり" 
}
```
- `comment`はnullable
- アップデート時も同じエンドポイントです

## Response

### 200 Ok
```
none
```