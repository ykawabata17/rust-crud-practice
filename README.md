# RustでRESTful API
Rust勉強のため，[Building REST APIs in Rust with Actix Web](https://www.vultr.com/docs/building-rest-apis-in-rust-with-actix-web/)を参考に実装してみた．  

# 使い方
以下のようなデータが最初に入っている．
```json
[{
  "id": 1,
  "author: "ReLU",
  },
  {
  "id": 2,
  "author: "Bob"
}]
```
このデータは`main`関数で編集できる．  
`cargo run`でサーバーを起動して，`curl`で`GET, POST, UPDATE, DELETE`を確かめることができる．  
以下はサーバー起動時のコマンド．  
* POST
```
> curl -X POST -H "Content-Type: application/json" -d '{"id": 3, "author": "Suzuki"}' http://localhost:8000/tickets
以下が出力される
-> {"id":3,"author":"Suzuki"}
```
* GET(All)
```
> curl http://localhost:8000/tickets
以下が出力される
-> [{"id":1,"author":"ReLU"},{"id":2,"author":"Bob"},{"id":3,"author":"Suzuki"}]
```
* UPDATE
```
> curl -XPUT 127.0.0.1:8000/tickets/1 -i -H "Content-Type: application/json" -d '{"id":1, "author":"Tanaka"}'
以下が出力される
-> 
HTTP/1.1 200 OK
content-length: 26
content-type: application/json
date: Fri, 19 May 2023 23:54:01 GMT
{"id":1,"author":"Tanaka"}
```

* GET{ID}
```
> curl -XGET 127.0.0.1:8000/tickets/1
以下が出力される
{"id":1,"author":"Tanaka"}
```

* DELETE
```
> curl -XDELETE -i 127.0.0.1:8000/tickets/3
以下が出力される
-> 
HTTP/1.1 200 OK
content-length: 26
content-type: application/json
date: Fri, 19 May 2023 23:56:21 GMT
{"id":3,"author":"Suzuki"}
```
