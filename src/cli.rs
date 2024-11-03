use clap::Parser;

use crate::app::RequestMethod;

#[derive(Parser)]
#[command(version, about, long_about=None, author)]
pub struct Cli {
    pub url: Option<String>,

    #[arg(default_value = "get", short = 'X', long, value_parser = clap::value_parser!(RequestMethod))]
    pub method: Option<RequestMethod>,
}
