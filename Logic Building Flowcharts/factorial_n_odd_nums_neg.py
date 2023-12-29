# Get user input for the value of 'n'
n = int(input("Enter n:"))

# Initialize variables for the loop
i = 3    
fact = 1 

# Execute the loop as long as 'i' is less than half of the square of 'n'
while i < ((n * n) / 2):
    
    print(f"{fact}")  

    # Update the factorial by multiplying with the product of three consecutive odd numbers
    fact *= i * (i - 1) * (-1)

    # Increment the counter variable by 2
    i += 2


