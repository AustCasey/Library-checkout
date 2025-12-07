# Library

## Run
```
cargo run
```

## Run all tests
```
cargo test
```

Encapsulation was enforced by using Rusts default private fields. Specifically in book and dvd you cannor directly access their fields, you must use the methods inside of the item trait. Composition was used by using traits. By implementing traits from item to book, book still owns its own data but can use methods from item. Polymorphism is used in catalogs get and add methods, which treat books and dvd the same. 