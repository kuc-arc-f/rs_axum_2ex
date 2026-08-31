# todo_2

 Version: 0.9.1

 date    : 2026/08/31

 update :

***

axum + Rust , API PostgreSQL example TODO

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
  -d '{"title":"TEST-DATA-001"}'

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
