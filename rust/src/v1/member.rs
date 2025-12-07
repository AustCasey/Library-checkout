use std::collections::HashSet;

pub struct Member {
    name: String,
    borrowed: HashSet<String>,
}

impl Member {
    pub const MAX_ITEMS: usize = 5;
    
    pub fn new(name: &str) -> Self {
        Member {
            name: name.to_string(),
            borrowed: HashSet::new(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn borrow(&mut self, item_id: &str) -> Result<(), String> {
        if self.borrowed.len() >= Self::MAX_ITEMS {
            return Err(format!("Cannot borrow more than {} items", Self::MAX_ITEMS));
        }
        
        if self.borrowed.contains(item_id) {
            return Err(format!("Item {} is already borrowed by this member", item_id));
        }
        
        self.borrowed.insert(item_id.to_string());
        Ok(())
    }
    
    pub fn return_item(&mut self, item_id: &str) -> Result<(), String> 
    {
        if !self.borrowed.contains(item_id) {
            return Err(format!("Item {} was not borrowed by this member", item_id));
        }
        
        self.borrowed.remove(item_id);
        Ok(())
    }
    
    pub fn borrowed_ids(&self) -> Vec<String> 
    {
        let mut ids: Vec<String> = self.borrowed.iter().cloned().collect();
        ids.sort();
        ids
    }
}