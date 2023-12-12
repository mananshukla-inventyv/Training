x=int(input("Enter x: "))
y=int(input("Enter y: "))
z=int(input("Enter z: "))

if(x>=y):
    if(x>=z):
        print(f"The greater num is: {x}")
    else:
        print(f"The greater num is: {z}")

else:
    if(y>=z):
        print(f"The greater num is: {y}")
    else:
        print(f"The greater num is: {z}")