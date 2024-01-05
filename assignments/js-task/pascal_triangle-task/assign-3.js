function findDigitCombinations(number) {
  if (number < 1000 || number > 9999) {
    alert("Please enter a valid 4-digit number.");
    return;
  }

  const digits = String(number).split("").map(Number);

  let combinations = [];

  // Single-digit combinations
  
  for (let i = 0; i < digits.length; i++) {
    combinations.push(digits[i]);
  }

  // Two-digit combinations
  for (let i = 0; i < digits.length; i++) {
    for (let j = 0; j < digits.length; j++) {
      if (i !== j) {
        combinations.push(digits[i] * 10 + digits[j]);
      }
    }
  }

  // Three-digit combinations
  for (let i = 0; i < digits.length; i++) {
    for (let j = 0; j < digits.length; j++) {
      for (let k = 0; k < digits.length; k++) {
        if (i !== j && i !== k && j !== k) {
          combinations.push(digits[i] * 100 + digits[j] * 10 + digits[k]);
        }
      }
    }
  }

  // Four-digit combinations
  for (let i = 0; i < digits.length; i++) {
    for (let j = 0; j < digits.length; j++) {
      for (let k = 0; k < digits.length; k++) {
        for (let l = 0; l < digits.length; l++) {
          if (i !== j && i !== k && i !== l && j !== k && j !== l && k !== l) {
            combinations.push(digits[i] * 1000 + digits[j] * 100 + digits[k] * 10 + digits[l]);
          }
        }
      }
    }
  }

  // Function to check if a number is prime
  function isPrime(num) {
    if (num <= 1) return false;
    for (let i = 2; i <= Math.sqrt(num); i++) {
      if (num % i === 0) {
        return false;
      }
    }
    return true;
  }

  // Filter out prime numbers from combinations
  const newSet=new Set(combinations)
  combinations=[...newSet]
  const primes = combinations.filter(isPrime);

  // console.log(`All combinations of digits in ${number}: ${combinations.join(", ")}`);
  
  return primes
}

// Example usage
// const userNumber = 3456;
// console.log(`Prime numbers in combinations: ${primes.join(", ")}`);
function generatePascalsTriangle(limit) {
  let triangle = [];
  let primes=findDigitCombinations(limit);
  let greatest_prime=primes[primes.length-1]
  let flag=1
  for (let i = 0; ; i++) {
      triangle[i] = [];
      if(flag==0){
        return triangle
      }
      triangle[i][0] = 1;
      
      for (let j = 1; j <= i; j++) {
              
          triangle[i][j] = triangle[i - 1][j - 1] + (triangle[i - 1][j] || 0);
          if (triangle[i][j] > greatest_prime) {
              flag=0
          }
      } 
  }
}

// function printPascalsTriangle(triangle) {
//   for (let i = 0; i < triangle.length; i++) {
//       let row = "";
//       for (let j = 0; j < triangle[i].length; j++) {
//           row += triangle[i][j] + " ";
//       }
//       console.log(row.trim());
//   }
// }

// const limit = primes[primes.length-1];
// const pascalsTriangle = generatePascalsTriangle(limit);
// printPascalsTriangle(pascalsTriangle);


function generatePascalsTriangleHTML(userNumber) {
  const triangle = generatePascalsTriangle(userNumber);
  const primes = findDigitCombinations(userNumber);
  const Cont = document.getElementById('right-main');
  const errorbox = document.getElementById('error-box');
  const container = document.createElement('div');
  container.style.textAlign = 'center';
  const notFoundPrimes = [];
  
  for (let i = 0; i < triangle.length; i++) {
    const row = triangle[i];
    const rowDiv = document.createElement('div');
    rowDiv.style.display = 'flex';
    rowDiv.style.justifyContent = 'center';
    
    for (let j = 0; j < row.length; j++) {
      const numberSpan = document.createElement('span');
      numberSpan.style.padding = '10px';
      
      // Check if the number is a prime and color it accordingly
      if (primes.includes(row[j])) {
        numberSpan.style.color = 'black';
        numberSpan.style.textDecoration = 'underline';
        numberSpan.style.backgroundColor = 'yellow';
        numberSpan.style.borderRadius = '50%'
      }
      
      numberSpan.textContent = row[j];
      rowDiv.appendChild(numberSpan);
    }
    container.appendChild(rowDiv);
  }
  Cont.appendChild(container)

  // Find prime numbers not present in Pascal's Triangle
  for (let i = 0; i < primes.length; i++) {
    if (!triangle.flat().includes(primes[i])) {
      notFoundPrimes.push(primes[i]);
    }
  }
  
  // Display prime numbers not found in Pascal's Triangle
  if (notFoundPrimes.length >= 0) {
    const errorDiv = document.createElement('div');
    const errorRatio = document.createElement('div');
    errorDiv.textContent = notFoundPrimes.length == 0?
    "Prime numbers not found in Pascal's Triangle: None":
    `Prime numbers not found in Pascal's Triangle: ${notFoundPrimes.join(', ')}`;
    errorDiv.style.marginTop = '20px';
    errorDiv.style.textAlign = 'center'; 
    errorRatio.textContent = notFoundPrimes.length == 0?
    "Error Ratio in pascals triangle is 0 %:":
    `Error Ratio in pascals triangle is : ${((notFoundPrimes.length / primes.length)*100).toFixed(2)} %`;
    errorRatio.style.marginTop = '20px';
    errorRatio.style.textAlign = 'center';
    errorbox.appendChild(errorDiv)
    errorbox.appendChild(errorRatio)
    }
  }
const btn = document.getElementById("btn")

btn.addEventListener('click', () => {
  const inputNumber = document.getElementById("inp").value;
  // let primes=findDigitCombinations(userNumber);
  const result = generatePascalsTriangle(inputNumber);
  
  // Clear existing content in the container
  const container = document.getElementById('right-main');
  container.innerHTML = ''; 
  
  const errorbox = document.getElementById('error-box');
  errorbox.innerHTML = ''; 
  // Generate Pascal's Triangle in HTML div elements and highlight primes
  generatePascalsTriangleHTML(inputNumber);
  
});