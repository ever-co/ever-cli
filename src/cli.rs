use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "ever", version, about = "Ever ecosystem CLI router")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Help,
    Version,
    List,
    Doctor,
    Install {
        product: String,
        #[arg(long = "from")]
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
    #[command(external_subcommand)]
    Route(Vec<String>),
}

impl Command {
    pub fn into_route_parts(self) -> Option<(String, Vec<String>)> {
        match self {
            Self::Route(parts) => {
                let mut iter = parts.into_iter();
                let product = iter.next()?;
                Some((product, iter.collect()))
            }
            _ => None,
        }
    }
}

impl From<Command> for (String, Vec<String>) {
    fn from(value: Command) -> Self {
        value
            .into_route_parts()
            .unwrap_or_else(|| (String::new(), Vec::new()))
    }
}
