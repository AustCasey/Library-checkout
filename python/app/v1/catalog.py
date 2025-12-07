
from .items import Item

class Catalog:
   
    
    def __init__(self) -> None:
        self._items: dict[str, Item] = {}
    
    def add(self, item: Item) -> None:
        if item.id.value in self._items:
            raise ValueError(f"Item with ID {item.id.value} already exists in catalog")
        self._items[item.id.value] = item
    
    def get(self, item_id: str) -> Item | None:
        return self._items.get(item_id)
    
    def contains(self, item_id: str) -> bool:
        return item_id in self._items
