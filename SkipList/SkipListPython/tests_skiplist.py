import unittest
from skiplist import SkipList

class TestSkipList(unittest.TestCase):
    def setUp(self):
        """Set up a new skip list before each test."""
        self.skip_list = SkipList()

    def test_insert_and_search(self):
        """Test insertion and search operations."""
        # Test inserting and searching for values
        values = [3, 6, 7, 9, 12, 19, 17, 26, 21, 25]
        for value in values:
            self.skip_list.insert(value)
            
        # Test successful searches
        for value in values:
            self.assertIsNotNone(
                self.skip_list.search(value),
                f"Should find value {value}"
            )
            
        # Test unsuccessful searches
        for value in [2, 4, 8, 10, 15]:
            self.assertIsNone(
                self.skip_list.search(value),
                f"Should not find value {value}"
            )

    def test_delete(self):
        """Test deletion operation."""
        # Insert some values
        values = [3, 6, 7, 9, 12]
        for value in values:
            self.skip_list.insert(value)
            
        # Test successful deletions
        for value in values:
            self.assertTrue(
                self.skip_list.delete(value),
                f"Should delete value {value}"
            )
            self.assertIsNone(
                self.skip_list.search(value),
                f"Should not find value {value} after deletion"
            )
            
        # Test unsuccessful deletions
        self.assertFalse(
            self.skip_list.delete(100),
            "Should return False when deleting non-existent value"
        )

    def test_empty_list(self):
        """Test operations on empty list."""
        self.assertIsNone(
            self.skip_list.search(1),
            "Search in empty list should return None"
        )
        self.assertFalse(
            self.skip_list.delete(1),
            "Delete from empty list should return False"
        )

    def test_duplicate_insert(self):
        """Test inserting duplicate values."""
        self.skip_list.insert(5)
        self.skip_list.insert(5)
        # Delete first occurrence
        self.assertTrue(self.skip_list.delete(5))
        # Should still find second occurrence
        self.assertIsNotNone(self.skip_list.search(5))
        # Delete second occurrence
        self.assertTrue(self.skip_list.delete(5))
        # Now should not find any occurrence
        self.assertIsNone(self.skip_list.search(5))

if __name__ == '__main__':
    unittest.main()

