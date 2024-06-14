use clap::{arg, Parser};

use crate::Error;

#[derive(Parser, Debug)]
pub struct QueryOpts {
    #[arg(short, long)]
    url: Option<String>,
}

pub fn query(opts: QueryOpts) -> Result<(), Error> {
    todo!()
}
