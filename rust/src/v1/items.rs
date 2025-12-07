pub trait Item {
    fn id(&self) -> &str;

    fn title(&self) -> &str;

    fn days_allowed(&self) -> u32;
    #[allow(dead_code)]
    fn description(&self) -> String {
        format!("{} ({} days)", self.title(), self.days_allowed())
    }
}

pub struct Book {
    id: String,
    title: String,
}

impl Book {
    pub fn new(id: &str, title: &str) -> Self {
        Book {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}

impl Item for Book {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn days_allowed(&self) -> u32 {
        14
    }

    fn description(&self) -> String {
        format!("{} ({} days)", self.title, self.days_allowed())
    }
}

pub struct Dvd {
    id: String,
    title: String,
}

impl Dvd {
    pub fn new(id: &str, title: &str) -> Self {
        Dvd {
            id: id.to_string(),
            title: title.to_string(),
        }
    }
}

impl Item for Dvd {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn days_allowed(&self) -> u32 {
        7
    }

    fn description(&self) -> String {
        format!("{} ({} days)", self.title, self.days_allowed())
    }
}
