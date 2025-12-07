from .catalog import Catalog


class Member:    
    MAX_ITEMS = 5
    
    def __init__(self, name: str) -> None:
        self._name, self._borrowed = name, set()
    
    @property
    def name(self) -> str:
        return self._name
    
    def borrow(self, item_id: str) -> None:
        if len(self._borrowed) >= self.MAX_ITEMS:
            raise ValueError(f"Cannot borrow more than {self.MAX_ITEMS} items")
        if item_id in self._borrowed:
            raise ValueError(f"Item {item_id} is already borrowed by this member")
        self._borrowed.add(item_id)
    
    def return_item(self, item_id: str) -> None:
        if item_id not in self._borrowed:
            raise ValueError(f"Item {item_id} was not borrowed by this member")
        self._borrowed.remove(item_id)
    
    def borrowed_ids(self) -> list[str]:
        return sorted(self._borrowed)
    
    def list_details(self, catalog: Catalog) -> list[str]:
        details = []
        for item_id in self.borrowed_ids():
            item = catalog.get(item_id)
            if item:
                details.append(item.description())
            else:
                details.append(f"Unknown item: {item_id}")
        return details
