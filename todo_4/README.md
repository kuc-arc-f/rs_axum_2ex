# todo_4

 Version: 0.9.1

 date    : 2026/08/30

 update :

***

axum + Rust , API SQLite example TODO

***
* rustc 1.97.1

***
* Test-code
* add

```
curl -X POST http://localhost:8080/todos -H "Content-Type: application/json" \
  -d '{"title":"coffee","content":"メモ","is_public":true, "food_orange":true,"food_apple":false,"food_banana":true,"pub_date":"2026-09-03","qty1":1,"qty2":2,"qty3":3}'

```

* list
```
curl http://localhost:8080/todos
```

* delete
```
curl -X DELETE http://localhost:8080/todos/1
```

***
