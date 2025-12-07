
# Library

## Install pytest
```
python3 -m pip install -U pytest
```

## Run
Use `-B` to not create cache directories 
```bash
python3 -B -m app
```

## Tests
```bash
python3 -m pytest
```
Encapsulation was enforced by using private variables (_name) in the different classes. This allows the main function to read those variables
but it is unable to directly modify them and must use the methods that are internal to the class to modify. This ensures that all rules are followed when making modifications.

Polymorphism was used in the catalog add function where add doesn't care if you add a book or dvd, it treats both just as an item. List_details also treats both as items and doesnt care about their specific types. 

I used inheritance for items creating abstract methods for the item class that book and dvd define for days allowed and their descriptions. This allows for adding new types of items that have unique days allowed without having to modify the items class.