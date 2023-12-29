# Get user input for the value of 'n'
n = int(input("Enter n: "))

# Initialize variables for the loop
i = 1  # Counter variable
w = 0  # Variable to store the sum

# Execute the loop as long as 'i' is less than or equal to 'n'
while i <= n:
    w = w + i  
    i = i + 1  

print(f"Sum is: {w}")
