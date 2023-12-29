# Get user input for the value of 'x'
x = int(input("Enter x: "))

# Get user input for the value of 'y'
y = int(input("Enter y: "))

# Get user input for the value of 'z'
z = int(input("Enter z: "))

# Check if 'x' is greater than or equal to 'y'
if x >= y:
    # If 'x' is greater than or equal to 'z', print 'x' as the greater number
    if x >= z:
        print(f"The greater num is: {x}")
    else:
        # If 'z' is greater than 'x', print 'z' as the greater number
        print(f"The greater num is: {z}")

else:
    # If 'y' is greater than 'x'
    # Check if 'y' is greater than or equal to 'z'
    if y >= z:
        print(f"The greater num is: {y}")
    else:
        # If 'z' is greater than 'y', print 'z' as the greater number
        print(f"The greater num is: {z}")
