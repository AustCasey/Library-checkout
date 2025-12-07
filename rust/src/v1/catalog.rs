use std::collections::HashMap;
use crate::v1::items::Item;

pub struct Catalog {
    items: HashMap<String, Box<dyn Item>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            items: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, item: Box<dyn Item>) -> Result<(), String> {
        let id = item.id().to_string();
        
        if self.items.contains_key(&id) {
            return Err(format!("Item with ID {} already exists in catalog", id));
        }
        
        self.items.insert(id, item);
        Ok(())
    }
    
    pub fn get(&self, item_id: &str) -> Option<&dyn Item> {
        self.items.get(item_id).map(|boxed| boxed.as_ref())
    }
    
    pub fn contains(&self, item_id: &str) -> bool {
        self.items.contains_key(item_id)
    }
    
    pub fn details_for(&self, item_ids: &[String]) -> Vec<(String, String, u32)> {
        item_ids
            .iter()
            .filter_map(|id| {
                self.get(id).map(|item| {
                    (id.clone(), item.title().to_string(), item.days_allowed())
                })
            })
            .collect()
    }
}