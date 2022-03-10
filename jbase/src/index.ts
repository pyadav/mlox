import { exit, argv, stdin, stdout } from "node:process";
import fs from "fs";
import { createInterface } from "readline";

async function main() {
  const args = argv.slice(2);

  if (args.length > 1) {
    console.error("Usage: jox [script]");
    exit(64);
  } else if (args.length === 1) {
    runFile(args[0]);
  } else {
    runPrompt();
  }
}

function runFile(path: string) {
  const contant = fs.readFileSync(path, { encoding: "utf8" });
  run(contant);
}

function runPrompt() {
  const rl = createInterface(stdin, stdout);
  rl.setPrompt("> ");
  rl.on("line", (line: string) => {
    if (line.trim() === "") {
      exit(0);
    }
    run(line);
  });
}

function run(data: string) {
  console.log(data);
}

main().catch((e) => {
  console.error(e.message);
  exit(-1);
});
