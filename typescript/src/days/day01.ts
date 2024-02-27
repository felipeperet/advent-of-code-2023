import fs from "fs";
import path from "path";

// Processes a line and returns the concatenated value of the first and last numbers.
function processLine(line: string): number {
  const numbers = line.match(/\d+/g);
  if (!numbers) return 0;

  const firstNumber = numbers[0];
  const lastNumber = numbers[numbers.length - 1];
  return parseInt(firstNumber + lastNumber, 10);
}

// Reads the file and calculates the sum.
function calculateSumFromFile(filePath: string) {
  fs.readFile(filePath, "utf8", (err, data) => {
    if (err) {
      console.error("Error reading file:", err);
      return;
    }

    const lines = data.split(/\r?\n/);
    let sum = 0;

    lines.forEach((line) => {
      sum += processLine(line);
    });

    console.log("Total sum:", sum);
  });
}

const filePath = path.join(__dirname, "../inputs/day01.txt");

calculateSumFromFile(filePath);
