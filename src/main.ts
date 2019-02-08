#!/usr/bin/env node

import yargs from "yargs";
const log = console.log;
import chalk from "chalk";

const error = chalk.bold.red;
const warning = chalk.keyword("orange");
const info = chalk.green;

log(info("Ever CLI"));

process.on("unhandledRejection", (err) => {
  error(err);
  process.exit(1);
});

