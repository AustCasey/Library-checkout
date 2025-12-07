mod v1;
 use v1::items::{Book, Dvd};
 use v1::catalog::Catalog;
 use v1::member::Member;
 fn main() {
    let mut cat = Catalog::new();
    cat.add(Box::new(Book::new("B1", "Rust for Humans"))).unwrap();
    cat.add(Box::new(Book::new("B2", "Pythonic Patterns"))).unwrap();
    cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
    cat.add(Box::new(Dvd::new("D2", "Patterns in Motion"))).unwrap();
    let mut m = Member::new("Clark");
    m.borrow("B1").unwrap();
    m.borrow("D1").unwrap();
    println!("{} has borrowed: {:?}", m.name(), m.borrowed_ids());
    for (idv, title, days) in cat.details_for(&m.borrowed_ids()) {
        println!(" • {} — {} ({} days)", idv, title, days);
    }
    m.return_item("B1").unwrap();
    println!("{} has now borrowed: {:?}", m.name(), m.borrowed_ids());
 }