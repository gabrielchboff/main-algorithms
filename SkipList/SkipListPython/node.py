class Node:
    """
    A node in a skip list data structure.
    Each node contains a value and maintains multiple forward references at different levels.
    """
    
    def __init__(self, value, level):
        """
        Initialize a skip list node.
        
        Args:
            value: The value stored in the node
            level: The number of forward references this node will maintain
        """
        self.value = value
        self.forward = [None] * (level + 1) 

    def __repr__(self):
        return f"Node(value={self.value}, forward={self.forward})"
        
    def __str__(self):
        return f"Node(value={self.value})"
