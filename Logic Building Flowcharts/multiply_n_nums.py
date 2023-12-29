# Get user input for the value of 'n'
n = int(input("Enter n: "))

# Initialize variables for the loop
i = 1 
w = 1 

# Execute the loop as long as 'i' is less than or equal to 'n'
while i <= n:
    w = w * i  
    i = i + 1  


print(f"Multiplication is: {w}")

