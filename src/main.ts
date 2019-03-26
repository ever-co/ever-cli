#!/usr/bin/env node

const log = console.log;
const clear = require("clear");

import yargs from "yargs";
import chalk, { Chalk } from "chalk";
import figlet from "figlet";

const error: Chalk = chalk.bold.red;
const warning: Chalk = chalk.keyword("orange");
const info: Chalk = chalk.green;

// if we want to clear console, call 'clear' below
// clear();

log(chalk.whiteBright(
  figlet.textSync("ever", { horizontalLayout: "default", font: "Graffiti" })
));

log("");

log(info("Ever CLI"));

process.on("unhandledRejection", (reason, p) => {
  log(error(`Unhandled Rejection at: ${p}, reason: ${reason}`));
  process.exit(1);
});

