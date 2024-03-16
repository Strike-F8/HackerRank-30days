if __name__ == '__main__':
    # Get the number of strings
    numStrings = int(input().strip())
    strings = list()

    # Get the strings
    for i in range(0,numStrings):
        strings.append(input().strip())

    # Divide the odd and even characters for each string
    
    for s in strings:
        evens = list()
        odds = list()

        for c in s:
            if len(evens) < len(odds):
                evens.append(c)
            else:
                odds.append(c)

        # print the results
        for c in odds:
            print(c, end="")
        print(end=" ")

        for c in evens:
            print(c, end="")
        print()
