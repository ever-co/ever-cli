#!/usr/bin/env node

import yargs from "yargs";
const log = console.log;
import chalk, { Chalk } from "chalk";

const error: Chalk = chalk.bold.red;
const warning: Chalk = chalk.keyword("orange");
const info: Chalk = chalk.green;

log(info("Ever CLI"));

process.on("unhandledRejection", (reason, p) => {
  log(error(`Unhandled Rejection at: ${p}, reason: ${reason}`));
  process.exit(1);
});

