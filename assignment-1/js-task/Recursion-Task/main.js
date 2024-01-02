// Function to calculate the combination (nCr)
function nCr(n, r) {
    if (r < 0 || r > n) {
        return 0; // Invalid input, return 0
    }

    // Calculate factorial of a number
    function factorial(num) {
        if (num === 0 || num === 1) {
            return 1;
        }
        return num * factorial(num - 1);
    }

    // Calculate nCr using the formula
    return factorial(n) / (factorial(r) * factorial(n - r));
}


function storeAndDisplayElements(array, index, storage) {
    // Base case: stop recursion when the index is beyond the array length
    if (index === array.length) {
      console.log("Stored and Displayed Elements:", storage);
      return;
    }
  
    // Store the current element in the storage array
    storage.push(array[index]);
  
    // Display the current element
    console.log(array[index]);
  
    // Recursive call to move to the next index
    storeAndDisplayElements(array, index + 1, storage);
  }

  


  const myArray = [1, 2, 3, 4, 5];

// Create an array to store elements
const storedElements = [];

var result1 = nCr(10,3);
console.log(result1);
// Call the combined function
storeAndDisplayElements(myArray, 0, storedElements);
var fibSeries=[];
var primes=[];
function fibonacciAndPrimes(n, current = 0, next = 1, fibSeries = []) {

    
  
    // Base case: Stop recursion when n terms are reached
    if (n === 0) {
        return fibSeries;
    }

    // Generate Fibonacci series
    fibSeries.push(current);

    // Check if the current Fibonacci number is prime
    if (isPrime(current)) {
        primes.push(current);
         // Output prime number
    }

    // Recursive call to generate the next term in the series
    return fibonacciAndPrimes(n - 1, next, current + next, fibSeries);
}

function isPrime(number) {
    if (number < 2) {
        return false; // 0 and 1 are not prime
    }

    // Check for divisibility up to the square root of the number
    for (let i = 2; i <= Math.sqrt(number); i++) {
        if (number % i === 0) {
            return false; // Number is divisible, not prime
        }
    }

    return true; // Number is prime
}

const n = 1001; // Change this to the desired number of terms
const result = fibonacciAndPrimes(n,0,1,[]);
console.log("Fibonacci Series:", result);
console.log("Prime Numbers:", primes);

// Calculate the difference between consecutive prime numbers
const primeDifferences = primes.slice(1).map((prime, index) => primes[index] - prime);
console.log("Prime Differences:", primeDifferences);
// Example usage

let sum = 0;

// Loop through each element in primeDifferences and add it to the sum
for (let i = 0; i < primeDifferences.length; i++) {
  sum += primeDifferences[i];
}

// Output the sum to the console
console.log("Sum of Prime Differences:", sum);
