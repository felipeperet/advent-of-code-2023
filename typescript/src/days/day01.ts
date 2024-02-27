import { readFileSync } from "fs";
import path from "path";

function processLine(line: string): number {
  const matches = line.match(/\d/g);
  if (matches && matches.length > 0) {
    const first = matches[0];
    const last = matches[matches.length - 1];
    return parseInt(first + last, 10);
  }
  return 0;
}

function sumCalibrationValues(filePath: string): number {
  const fileContent = readFileSync(filePath, "utf-8");
  const lines = fileContent.split("\n");

  let sum = 0;
  lines.forEach((line) => {
    sum += processLine(line);
  });

  return sum;
}

const filePath = path.join(__dirname, "../inputs/day01.txt");
const totalSum = sumCalibrationValues(filePath);
console.log(`Total Sum: ${totalSum}`);
