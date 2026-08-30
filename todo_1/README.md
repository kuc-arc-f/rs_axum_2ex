# todo_1

 Version: 0.9.1

 date    : 2026/08/30

 update :

***

axum + Rust , API SQLite example TODO

***
* rustc 1.97.1

***
### front

https://github.com/kuc-arc-f/tanstack_start_4ex/tree/main/todo_4

***
* Test-code
* add

```
curl -X POST http://localhost:8080/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "TEST-DATA-001"}'

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
