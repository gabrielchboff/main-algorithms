import random
from node import Node

class SkipList:
    """
    A skip list implementation providing O(log n) average time complexity
    for search, insertion, and deletion operations.
    """
    
    MAX_LEVEL = 30  # Maximum level for this skip list
    P = 0.5        # P value for level generation (0.5 is optimal)
    
    def __init__(self):
        """Initialize an empty skip list."""
        # Create header node and initialize level
        self.header = Node(float('-inf'), self.MAX_LEVEL)
        self.level = 0

    def _random_level(self):
        """
        Randomly generate node level using p = 0.5
        """
        level = 0
        while random.random() < self.P and level < self.MAX_LEVEL:
            level += 1
        return level
    
    def insert(self, value):
        """
        Insert a new value into the skip list
        
        Args:
            value: Value to insert
        """
        # Array to store update positions
        update = [None] * (self.MAX_LEVEL + 1)
        current = self.header
        
        # Start from highest level and work down
        for i in range(self.level, -1, -1):
            while (current.forward[i] and 
                   current.forward[i].value < value):
                current = current.forward[i]
            update[i] = current
            
        # Generate random level for new node
        new_level = self._random_level()
        
        # If new level is greater than current
        if new_level > self.level:
            for i in range(self.level + 1, new_level + 1):
                update[i] = self.header
            self.level = new_level
            
        # Create new node and update forward references
        new_node = Node(value, new_level)
        for i in range(new_level + 1):
            new_node.forward[i] = update[i].forward[i]
            update[i].forward[i] = new_node

    def search(self, value):
        """
        Search for a value in the skip list
        
        Args:
            value: Value to search for
            
        Returns:
            Node if found, None if not found
        """
        current = self.header
        
        # Start from highest level and work down
        for i in range(self.level, -1, -1):
            while (current.forward[i] and 
                   current.forward[i].value < value):
                current = current.forward[i]
                
        # May have found it
        current = current.forward[0]
        
        if current and current.value == value:
            return current
        return None
    
    def delete(self, value):
        """
        Delete a value from the skip list
        
        Args:
            value: Value to delete
            
        Returns:
            bool: True if value was deleted, False if not found
        """
        update = [None] * (self.MAX_LEVEL + 1)
        current = self.header
        
        # Find all levels where value occurs
        for i in range(self.level, -1, -1):
            while (current.forward[i] and 
                   current.forward[i].value < value):
                current = current.forward[i]
            update[i] = current
            
        current = current.forward[0]
        
        # If current node contains value
        if current and current.value == value:
            # Start from lowest level and clear references
            for i in range(self.level + 1):
                if update[i].forward[i] != current:
                    break
                update[i].forward[i] = current.forward[i]
                
            # Update list level
            while self.level > 0 and not self.header.forward[self.level]:
                self.level -= 1
            return True
        return False
    
    def display(self):
        """
        Display the skip list structure
        """
        for level in range(self.level, -1, -1):
            print(f"Level {level}: ", end="")
            node = self.header.forward[level]
            while node:
                print(f"{node.value} ", end="")
                node = node.forward[level]
            print()

