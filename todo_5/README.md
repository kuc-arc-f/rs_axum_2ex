# todo_5

 Version: 0.9.1

 date    : 2026/08/31

 update :

***

axum + Rust , API PostgreSQL example TODO

***
* rustc 1.97.1

***
* .env

```
DATABASE_URL=postgresql://root:admin@localhost/mydb
```
***
* Table: table.sql

***
* Test-code
* add

```
curl -X POST http://localhost:8080/todos \
  -H "Content-Type: application/json" \
  -d '{
    "title": "coffee",
    "content": "牛乳",
    "is_public": true,
    "food_orange": true,
    "food_apple": false,
    "food_banana": false,
    "pub_date": "2025-09-03",
    "qty1": 1, "qty2": 2, "qty3": 3
  }'

```

* list
```
curl http://localhost:8080/todos
```

* delete
```
curl -X DELETE http://localhost:8080/todos/<id>
```

***
