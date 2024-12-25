def get_keys_locks(): 
    keys = [] 
    locks = [] 

    keys_heights = [] 
    locks_heights = []

    with open("25/input.txt", "r") as file: 
        sequence = []
        for line in file:
            line = line.strip()
            if line:
                sequence.append(line)
            else:
                if '#' in sequence[-1]:
                    keys.append(sequence)
                else:
                    locks.append(sequence)
                sequence = []
    
    return keys, locks


def get_heights(sequence):
    heights = [] 
    for i in range(len(sequence)):
        for j in range(len(sequence[i])):
            if sequence[i][j] == '#':
                heights.append((i, j))
    return heights


main() 