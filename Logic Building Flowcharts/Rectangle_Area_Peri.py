# Get user input for the length of the Rectangle
l = int(input("Enter the length of the Rectangle: "))

# Get user input for the breadth of the Rectangle
b = int(input("Enter the breadth of the Rectangle: "))

# Finding Area: Multiply length and breadth
Area = l * b

# Finding Perimeter: Add twice the length and breadth
Perimeter = 2 * (l + b)

# Display the calculated area and perimeter
print(f"The area of the Rectangle is: {Area} (m²)")
print(f"The perimeter of the Rectangle is: {Perimeter} (meters)")
