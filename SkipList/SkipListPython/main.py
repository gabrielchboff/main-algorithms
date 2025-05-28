import random
import time
from skiplist import SkipList

skip_list = SkipList()

list_of_ids = [_ for _ in range(10000000)]

start_time = time.time()
for current_id in list_of_ids:
    skip_list.insert(current_id)
end_time = time.time()

print(f"Time taken to insert 10 million elements: {end_time - start_time} seconds")

# skip_list.display()

start_time = time.time()
x = skip_list.search(99)
end_time = time.time()

print(f"Time taken to search for 99: {end_time - start_time} seconds")
print(f"Search for 99: {x}")

start_time = time.time()
y = skip_list.search(10000)
end_time = time.time()

print(f"Time taken to search for 10000: {end_time - start_time} seconds")
print(f"Search for 10000: {y}")

start_time = time.time()
z = skip_list.search(21976)
end_time = time.time()

print(f"Time taken to search for 21976: {end_time - start_time} seconds")
print(f"Search for 21976: {z}")

start_time = time.time()
large_number = skip_list.search(1000000)
end_time = time.time()
print(f"Time taken to search for 1000000: {end_time - start_time} seconds")
print(f"Search for 1000000: {large_number}")

start_time = time.time()
for ids in list_of_ids:
    skip_list.delete(ids)
end_time = time.time()

print(f"Time taken to delete 10 million elements: {end_time - start_time} seconds")
