#!/usr/bin/env node

const log: any = console.log;
import chalk, { Chalk } from "chalk";
import clear from "clear";
import figlet from "figlet";
import yargs from "yargs";

const error: Chalk = chalk.bold.red;
const warning: Chalk = chalk.keyword("orange");
const info: Chalk = chalk.green;

// if we want to clear console, call 'clear' below
clear();

log(chalk.whiteBright(
  figlet.textSync("ever", {
    font: "Graffiti",
    horizontalLayout: "default",
  }),
));

log("");

log(info("Ever CLI"));

process.on("unhandledRejection", (reason, p) => {
  log(error(`Unhandled Rejection at: ${p}, reason: ${reason}`));
  process.exit(1);
});
