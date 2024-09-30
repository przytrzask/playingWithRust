import fs from "fs";

// function main() {
//   const list = [1, 2, 3].map((item) => item + 1);

//   console.log(list);
// }

// main();

// async function readFile() {
//   const file = fs.readFileSync("lines");

//   file
//     .toString()
//     .split("\n")
//     .filter((_l, index) => index % 2 === 0)
//     .forEach((line) => console.log(line));
// }

// readFile();

// enum Color {
//   Green = "Green",
//   Blue = "Blue",
//   Red = "Red",
// }

// function printColor(color: Color) {
//   switch (color) {
//     case Color.Red:
//       console.log("red");
//       break;
//     case Color.Green:
//       console.log("green");
//       break;
//     case Color.Blue:
//       console.log("blue");
//       break;
//   }
// }

// printColor(Color.Green);

// type Custom = {
//   age: number;
//   name: string;
// };

// type Item = number | string | Custom;

// function append(items: Item[]) {
//   items.push("hello fem");
// }

// const items: Item[] = [];

// console.log(items);

// append(items);
// console.log(items);

// const numbers: number[] = [];

// append(numbers);
// console.log(items);
//
//
function option(take: number | undefined): number {
  if (typeof take === "number") {
    return take * 5;
  } else {
    return 0;
  }
}

function practice(list: number[], idx: number): number {
  return (list[idx] ?? idx) * 5;
}

function numbers() {
  const file = process.argv[2];
  console.log({ file });

  fs.readFileSync(process.argv[2])
    .toString()
    .split("\n")
    .forEach((line) => console.log(line));
}

// numbers();
//

interface Area {
  area: () => number;
}

class Rectangle implements Area {
  constructor(
    public x: number,
    public y: number,
    public height: number,
    public width: number,
  ) {}
  area(): number {
    return this.height * this.width;
  }
}

class Circle {
  constructor(
    public x: number,
    public y: number,
    public radius: number,
  ) {}

  area(): number {
    return this.radius * this.radius * Math.PI;
  }
}
