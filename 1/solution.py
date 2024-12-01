import numpy as np

def main():
    # Read input
    array = []
    with open("test_input.txt", "r") as file:
        for line in file: 
            array.append([int(n) for n in line.split()])



    return array

if __name__ == "__main__":
    print(main())
        