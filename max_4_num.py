w=int(input("Enter w: "))
x=int(input("Enter x: "))
y=int(input("Enter y: "))
z=int(input("Enter z: "))

if(w>=x):

    if(w>=y):
        if(w>=z):
            print(f"The greater num is: {w}")
        else:
            print(f"The greater num is {z}")

    elif(y>=z):
        print(f"The greater num is: {y}")

    else:
        print(f"The greater num is: {z}")

else:
    if(x>=y):
        if(x>=z):
            print(f"The greater num is: {x}")
        else:
            print(f"The greater num is: {z}")
    
    elif(y>=z):
        print(f"The greater num is: {y}")
    
    else:
        print(f"The greater num is: {z}")