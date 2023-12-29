# Get user input for the value of 'w'
w = int(input("Enter w: "))

# Get user input for the value of 'x'
x = int(input("Enter x: "))

# Get user input for the value of 'y'
y = int(input("Enter y: "))

# Get user input for the value of 'z'
z = int(input("Enter z: "))

# Check if 'w' is greater than or equal to 'x'
if w >= x:
    # If 'w' is greater than or equal to 'y'
    if w >= y:
        # If 'w' is greater than or equal to 'z', print 'w' as the greater number
        if w >= z:
            print(f"The greater num is: {w}")
        else:
            # If 'z' is greater than 'w', print 'z' as the greater number
            print(f"The greater num is: {z}")
    # If 'y' is greater than 'w'
    elif y >= z:
        # If 'y' is greater than or equal to 'z', print 'y' as the greater number
        print(f"The greater num is: {y}")
    else:
        # If 'z' is greater than 'y', print 'z' as the greater number
        print(f"The greater num is: {z}")
# If 'x' is greater than 'w'
else:
    # If 'x' is greater than or equal to 'y'
    if x >= y:
        # If 'x' is greater than or equal to 'z', print 'x' as the greater number
        if x >= z:
            print(f"The greater num is: {x}")
        else:
            # If 'z' is greater than 'x', print 'z' as the greater number
            print(f"The greater num is: {z}")
    # If 'y' is greater than 'x'
    elif y >= z:
        # If 'y' is greater than or equal to 'z', print 'y' as the greater number
        print(f"The greater num is: {y}")
    else:
        # If 'z' is greater than 'y', print 'z' as the greater number
        print(f"The greater num is: {z}")
