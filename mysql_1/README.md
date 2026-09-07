# mysql_1

 Version: 0.9.1

 date    : 2026/09/07

 update :

***

axum + Rust , API MySQL  example TODO

* rustc 1.97.1
* MySQL 8.4.0

***
### Related - client

https://github.com/kuc-arc-f/cpp_15ex/tree/main/express_cl_5

***
* .env

```
DATABASE_URL=mysql://root:admin@127.0.0.1:3306/mydb
SERVER_ADDR=0.0.0.0:3000

```
***
* Table: table.sql

***
* Test-code
* add

```
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "TEST-DATA-001"}'

```

* list
```
curl http://localhost:3000/todos
```

* delete
```
curl -X DELETE http://localhost:3000/todos/1
```

***
