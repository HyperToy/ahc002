N = int(input())

sum = 0
for i in range(0, N):
    line = input().split(" ")
    while line[0] != "Score":
        line = input().split(" ")
    score = int(line[2])
    if score == 0:
        score = 1000000000
    sum += score

print("case: {}".format(N))
print("sum : {0:,}".format(sum))
print("avg : {0:,}".format(int(sum / N)))
