console.time("JS Execution Time");

let sum = 0;
for (let i = 0; i < 100_000_000; i++) {
    sum += i;
}

console.timeEnd("JS Execution Time");
console.log("Sum:", sum); 