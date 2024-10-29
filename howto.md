# API HOW TO
- `"/get_tag"`
```
curl http://localhost:8081/get_tag\?f\=0\&r\=1
[{"id":1,"name":"programming"}]
``` 

- `"/search"`
```
curl http://localhost:8081/search\?q\="program"
[{"book":{"id":1,"title":"C: The Complete Reference, 4th Ed","author":"Herbert Schildt","desc":"Another gem from Herb Schildt--best-selling programming author with more than 2.5 million books sold! C: The Complete Reference, Fourth Edition gives you full details on C99, the New ANSI/ISO Standard for C. You will get in-depth coverage of the C language and function libraries as well as all the newest C features, including restricted pointers, inline functions, variable-length arrays, and complex math. This jam-packed resource includes hundreds of examples and sample applications.","tags":[{"id":1,"name":"programming"},{"id":2,"name":"clang"},{"id":3,"name":"c99"}],"year":"2000","cover":""},"score":0.20412414523193148}]
```

- `"/get_book_info"`
```
curl http://localhost:8081/get_book_info\?id\=1
{"id":1,"title":"C: The Complete Reference, 4th Ed","author":"Herbert Schildt","desc":"Another gem from Herb Schildt--best-selling programming author with more than 2.5 million books sold! C: The Complete Reference, Fourth Edition gives you full details on C99, the New ANSI/ISO Standard for C. You will get in-depth coverage of the C language and function libraries as well as all the newest C features, including restricted pointers, inline functions, variable-length arrays, and complex math. This jam-packed resource includes hundreds of examples and sample applications.","tags":[{"id":1,"name":"programming"},{"id":2,"name":"clang"},{"id":3,"name":"c99"}],"year":"2000","cover":""}
```

- `"/get_book_from_tag"`
```
curl http://localhost:8081/get_book_from_tag\?f\=0\&r\=10\&id\="1"
[{"id":1,"title":"C: The Complete Reference, 4th Ed","author":"Herbert Schildt","desc":"Another gem from Herb Schildt--best-selling programming author with more than 2.5 million books sold! C: The Complete Reference, Fourth Edition gives you full details on C99, the New ANSI/ISO Standard for C. You will get in-depth coverage of the C language and function libraries as well as all the newest C features, including restricted pointers, inline functions, variable-length arrays, and complex math. This jam-packed resource includes hundreds of examples and sample applications.","tags":[{"id":1,"name":"programming"},{"id":2,"name":"clang"},{"id":3,"name":"c99"}],"year":"2000","cover":""}]
```

- `"/add_book"`
```
BLM JADI
```

- `"/add_tag"`
```
BLM JADI
```

- `"/del_tag"`
```
curl http://localhost:8081/del_tag\?id\=3
"Ok": "SUCCESS"
```

- `"/del_book"`
```
curl http://localhost:8081/del_book\?id\=3
"Ok": "SUCCESS"
```

## NOTE
- add : `sort="asc" to sort ascending when calling`
- add : `sort="desc" to sort descending when calling`
