n=int(input("Enter n:"))
i=3
fact=1

while(i<((n*n)/2)):
    print(f"{fact}")

    fact*=i*(i-1)*(-1)
    i+=2

