#[cfg(test)]
mod tests {
    use crate::v1::catalog::Catalog;
    use crate::v1::items::{Book, Dvd};
    use crate::v1::member::Member;

    fn setup_sample() -> Catalog {
        let mut cat = Catalog::new();
        cat.add(Box::new(Book::new("B1", "Rust for Humans")))
            .unwrap();
        cat.add(Box::new(Book::new("B2", "Pythonic Patterns")))
            .unwrap();
        cat.add(Box::new(Dvd::new("D1", "Taking Flight"))).unwrap();
        cat
    }

    #[test]
    fn test_add_and_get() {
        let cat = setup_sample();
        assert_eq!(cat.get("B1").unwrap().title(), "Rust for Humans");
        assert_eq!(cat.get("D1").unwrap().days_allowed(), 7);
    }

    #[test]
    fn test_duplicate_id_rejected() {
        let mut cat = setup_sample();
        let result = cat.add(Box::new(Book::new("B1", "Duplicate")));
        assert!(result.is_err());
    }

    #[test]
    fn test_member_borrow_and_return() {
        let mut m = Member::new("Clark");

        m.borrow("B1").unwrap();
        assert_eq!(m.borrowed_ids(), vec!["B1"]);

        m.return_item("B1").unwrap();
        assert_eq!(m.borrowed_ids(), Vec::<String>::new());
    }

    #[test]
    fn test_member_cannot_borrow_twice() {
        let mut m = Member::new("X");
        m.borrow("K1").unwrap();

        let result = m.borrow("K1");
        assert!(result.is_err());
    }

    #[test]
    fn test_member_limit_enforced() {
        let mut m = Member::new("X");
        for i in 0..5 {
            m.borrow(&format!("K{}", i)).unwrap();
        }

        let result = m.borrow("K5");
        assert!(result.is_err());
    }

    #[test]
    fn test_member_return_unborrowed_item() {
        let mut m = Member::new("Y");

        let result = m.return_item("K2");
        assert!(result.is_err());
    }

    #[test]
    fn test_details_lines() {
        let cat = setup_sample();
        let mut m = Member::new("Y");
        m.borrow("B2").unwrap();

        let details = cat.details_for(&m.borrowed_ids());
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].1, "Pythonic Patterns");
        assert_eq!(details[0].2, 14);
    }

    #[test]
    fn test_return_item_twice() {
        let mut m = Member::new("Z");
        m.borrow("K3").unwrap();
        m.return_item("K3").unwrap();

        let result = m.return_item("K3");
        assert!(result.is_err());
    }
}
