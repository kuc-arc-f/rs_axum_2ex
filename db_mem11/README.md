# db_mem11

 Version: 0.9.1

 date    : 2026/10/13
 
 update :

***

axum API , memory DB SQLite TODO

* rustc 1.97.1

***
### related DB Server

https://github.com/kuc-arc-f/rs_db_mem_rest

***
* table add
```
sqlite3 ./data/backup.db < table.sql
```

* table TODO
```
sqlite3 ./data/backup.db < todo.sql
```

***
* build
```
cargo build --release
```

* start server
```
./start.sh
```

***