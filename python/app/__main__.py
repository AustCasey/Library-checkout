
from app.v1.items import Id, Book, Dvd
from app.v1.catalog import Catalog
from app.v1.member import Member

def demo() -> None:
    cat = Catalog()

    # Books allowed 7 days
    cat.add(Book(Id("B1"), "Rust for Humans"))
    cat.add(Book(Id("B2"), "Pythonic Patterns"))

    # Dvds allowed 14 days
    cat.add(Dvd(Id("D1"), "Taking Flight"))
    cat.add(Dvd(Id("D2"), "Patterns in Motion"))

    m = Member("Clark")
    m.borrow("B1")
    m.borrow("D1")
    print(m.name, "has borrowed:", m.borrowed_ids())
    for line in m.list_details(cat):
        print(" •", line)
    m.return_item("B1")
    print(m.name, "has borrowed:", m.borrowed_ids())

if __name__ == "__main__":
    demo()
