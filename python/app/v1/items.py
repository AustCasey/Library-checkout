from abc import ABC, abstractmethod
from dataclasses import dataclass

@dataclass(frozen=True, slots=True)
class Id:
    value: str

class Item(ABC):
    def __init__(self, id: Id, title: str) -> None:
        self._id, self._title = id, title

    @property
    def id(self) -> Id: 
        return self._id

    @property
    def title(self) -> str: 
        return self._title

    @abstractmethod
    def days_allowed(self) -> int: ...

    @abstractmethod 
    def description(self) -> str: ...

class Book(Item):
    def days_allowed(self) -> int: 
        return 14
    def description(self) -> str:
        return f"{self.title} ({self.days_allowed()} days)"

class Dvd(Item):
    def days_allowed(self) -> int: 
        return 7
    def description(self) -> str:
        return f"{self.title} ({self.days_allowed()} days)"
