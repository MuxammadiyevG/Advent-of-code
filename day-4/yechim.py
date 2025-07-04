with open("input.txt", "r") as f:
    raw_data = f.readlines()
arr = []
for i in raw_data:
    arr.append(i.replace("\n", ""))

word="XMAS"
count = 0
rows = len(arr)
cols = len(arr[0])

for i in range(rows):
    for j in range(cols):
        if j + 3 < cols:
            if (
                arr[i][j] == "X"
                and arr[i][j + 1] == "M"
                and arr[i][j + 2] == "A"
                and arr[i][j + 3] == "S"
            ):
                count += 1

        if j - 3 >= 0:
            if (
                arr[i][j] == "X"
                and arr[i][j - 1] == "M"
                and arr[i][j - 2] == "A"
                and arr[i][j - 3] == "S"
            ):
                count += 1

        if i + 3 < rows:
            if (
                arr[i][j] == "X"
                and arr[i + 1][j] == "M"
                and arr[i + 2][j] == "A"
                and arr[i + 3][j] == "S"
            ):
                count += 1

        if i - 3 >= 0:
            if (
                arr[i][j] == "X"
                and arr[i - 1][j] == "M"
                and arr[i - 2][j] == "A"
                and arr[i - 3][j] == "S"
            ):
                count += 1

        if i + 3 < rows and j + 3 < cols:
            if (
                arr[i][j] == "X"
                and arr[i + 1][j + 1] == "M"
                and arr[i + 2][j + 2] == "A"
                and arr[i + 3][j + 3] == "S"
            ):
                count += 1

        if i - 3 >= 0 and j - 3 >= 0:
            if (
                arr[i][j] == "X"
                and arr[i - 1][j - 1] == "M"
                and arr[i - 2][j - 2] == "A"
                and arr[i - 3][j - 3] == "S"
            ):
                count += 1

        if i + 3 < rows and j - 3 >= 0:
            if (
                arr[i][j] == "X"
                and arr[i + 1][j - 1] == "M"
                and arr[i + 2][j - 2] == "A"
                and arr[i + 3][j - 3] == "S"
            ):
                count += 1

        if i - 3 >= 0 and j + 3 < cols:
            if (
                arr[i][j] == "X"
                and arr[i - 1][j + 1] == "M"
                and arr[i - 2][j + 2] == "A"
                and arr[i - 3][j + 3] == "S"
            ):
                count += 1

print(count)
