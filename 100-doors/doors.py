doors = [0] * 100

for i in range(100):
    for j in range(i, 100, i + 1):
        doors[j] = not doors[j]  # Toggle the state of the door at index j
    state = 'open' if doors[i] else 'close'
    print(f'Door {i}: {state}')
