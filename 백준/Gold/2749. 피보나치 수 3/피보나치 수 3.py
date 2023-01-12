N = int(input())

mod = 1000000
fibo = [0, 1]
p = mod//10*15

for i in range(2,p):
    fibo.append(fibo[i-1]+fibo[i-2])
    fibo[i] %= mod

print(fibo[N%p])