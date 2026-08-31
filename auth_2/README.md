# auth_2

 Version: 0.9.1

 date    : 2026/08/30

 update :

***

axum + Rust , Auth PostgreSQL example , signup Login

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
* signup

```
curl -X POST http://localhost:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "you3@example.com", "password": "1111"}'

```

* login
```
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"email": "you@example.com", "password": "1111"}'

```

***
