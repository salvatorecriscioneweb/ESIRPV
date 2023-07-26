## ESIRPV 
**E**commerce **S**imulation **I**n **R**ust **P**ython(Django) and **V**ue3

## What is used for?

| --      | Rust | Django | Vue3 |   |
|---------|------|--------|------|---|
| Label Generation PDF   | X    |        |      |   |
| Code128 Generation | X    |        |      |   |
| QR Code Generation | X    |        |      |   |
| DB      |      | X      |      |   |
| UI      |      |        | X    |   |


## Run it!

Have available [rust](https://www.rust-lang.org/learn/get-started) in your OS. Otherwise pre-built binary will be available for MacOS ARM, ~~Windows, MacOS x86, and Unix~~. ( TODO )

### rust-generate-flush-db
This is a utility binary, built because much much much faster than Django Python interpreter. Django will rely on this for "longer" operations.

**Skip this part if you will use pre-built binary**

`cd rust-generate-flush-db`

Build and copy:
`cargo build && target/debug/rust-generate-flush-db ../backend/backend/backend/quickstart/utils/rust-generate-flush-db`

##### CMD(s)

~~`rust-generate-flush-db flush db.sqlite`
Remove documents ( and delete the files ) of orders older than 1 days ( they can be regenerate later )~~ TODO: Move to Django

`rust-generate-flush-db generate [order] [min_prod] [max_prod] db.sqlite`
Generate dataset in the database. DEMO DB

`rust-generate-flush-db digest [file] db.sqlite`
Digest the file using fast sha256, this is useful for 

`rust-generate-flush-db document [client_data] [order_id] db.sqlite`
Create label in PDF including QRCode and Barcode using Code128 Charset A.

`rust-generate-flush-db version`
Return the version. Useful to check if the binary works correctly.


### Backend 

Python version `3.9.6`
Requirements:
`pip install django`
`pip install djangorestframework`

##### Routes:
**Login**:
*Request*
`POST /api-token-auth/ | Formdata(username, password)` 
Response
```
{
	"token": String
}
```

**Generate Document**:
Request
`GET /document?orderId=ABC123` 
Response
FILE Binary PDF
![Route Screenshot](https://raw.githubusercontent.com/salvatorecriscioneweb/ESIRPV/main/images/document_print.png)
**Get** **Orders**:
Request:
`GET /get-orders`
Response:
```
{
	"orders":  [
		{
			"order":  {
				"id":  1,
				"order_id":  "ABC123",
				"address":  "Address, Tallinn 10026",
				"client_id":  "1",
				"timestamp":  "2023-07-26T19:45:57Z"
			},
		"list":  [
			{
				"id":  1,
				"order_id":  1,
				"number":  "1",
				"product_id":  1,
				"quantity":  4
			}
		],
		"client":  [
			{
				"name":  "Salvatore",
				"surname":  "Criscione",
				"id":  "1",
				"address":  "Address, Tallinn 10026"
				}
		]
	}
]}```

TODO!

### Frontend

TODO!
