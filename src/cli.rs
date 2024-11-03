use clap::Parser;

use crate::app::RequestMethod;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Cli {
    pub endpoint: Option<String>,

    #[arg(value_enum, default_value = "get", short, long)]
    pub method: Option<RequestMethod>,
}
