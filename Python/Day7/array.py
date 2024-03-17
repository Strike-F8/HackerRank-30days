if __name__ == '__main__':
    n = int(input().strip())

    arr = list(map(int, input().rstrip().split()))

    # Reverse the array
    arr.reverse()

    # Print the contents of the array
    for i in arr:
        print(i, end=" ")
