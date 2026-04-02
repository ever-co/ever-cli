use crate::error::{RouterError, RouterResult};

#[derive(Debug)]
pub struct Cli {
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    Help,
    Version,
    List,
    Doctor,
    Install {
        product: String,
        source: Option<String>,
    },
    Uninstall {
        product: String,
    },
    Update {
        product: Option<String>,
    },
    Config {
        args: Vec<String>,
    },
    Route {
        product: String,
        args: Vec<String>,
    },
}

impl Cli {
    pub fn parse(args: Vec<String>) -> RouterResult<Self> {
        if args.is_empty() {
            return Ok(Self {
                command: Command::Help,
            });
        }

        let first = args[0].as_str();
        let rest = args[1..].to_vec();

        let command = match first {
            "--help" | "-h" | "help" => Command::Help,
            "--version" | "-V" | "version" => Command::Version,
            "list" => Command::List,
            "doctor" => Command::Doctor,
            "install" => parse_install(rest)?,
            "uninstall" => parse_uninstall(rest)?,
            "update" => parse_update(rest)?,
            "config" => Command::Config { args: rest },
            product => Command::Route {
                product: product.to_string(),
                args: rest,
            },
        };

        Ok(Self { command })
    }
}

impl Command {
    pub fn into_route_parts(self) -> Option<(String, Vec<String>)> {
        match self {
            Self::Route { product, args } => Some((product, args)),
            _ => None,
        }
    }
}

fn parse_install(args: Vec<String>) -> RouterResult<Command> {
    if args.is_empty() {
        return Err(RouterError::Message(
            "Missing product name. Usage: ever install <product> [--from npm]".to_string(),
        ));
    }

    let mut product: Option<String> = None;
    let mut source: Option<String> = None;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--from" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    RouterError::Message("Missing value for --from".to_string())
                })?;
                source = Some(value.clone());
                index += 2;
            }
            value if value.starts_with("--from=") => {
                source = Some(value.trim_start_matches("--from=").to_string());
                index += 1;
            }
            value if product.is_none() => {
                product = Some(value.to_string());
                index += 1;
            }
            value => {
                return Err(RouterError::Message(format!(
                    "Unexpected argument for install: {value}"
                )));
            }
        }
    }

    Ok(Command::Install {
        product: product.ok_or_else(|| {
            RouterError::Message(
                "Missing product name. Usage: ever install <product> [--from npm]".to_string(),
            )
        })?,
        source,
    })
}

fn parse_uninstall(args: Vec<String>) -> RouterResult<Command> {
    let product = args.first().ok_or_else(|| {
        RouterError::Message("Missing product name. Usage: ever uninstall <product>".to_string())
    })?;

    if args.len() > 1 {
        return Err(RouterError::Message(
            "Unexpected extra arguments for uninstall".to_string(),
        ));
    }

    Ok(Command::Uninstall {
        product: product.clone(),
    })
}

fn parse_update(args: Vec<String>) -> RouterResult<Command> {
    if args.len() > 1 {
        return Err(RouterError::Message(
            "Usage: ever update [product]".to_string(),
        ));
    }

    Ok(Command::Update {
        product: args.first().cloned(),
    })
}
