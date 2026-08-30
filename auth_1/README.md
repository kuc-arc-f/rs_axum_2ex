# auth_1

 Version: 0.9.1

 date    : 2026/08/30

 update :

***

axum + Rust , Auth example SQLite , signup Login

***
* rustc 1.97.1

***
### front

https://github.com/kuc-arc-f/tanstack_start_4ex/tree/main/auth_4

***
* Test-code
* signup

```
curl -X POST http://localhost:8080/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "you@example.com", "password": "1111"}'

```

* login
```
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/json" \
  -d '{"email": "you@example.com", "password": "1111"}'

```

***
