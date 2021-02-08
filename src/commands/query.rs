use clap::Clap;

use crate::Error;

#[derive(Clap, Debug)]
pub struct QueryOpts {
    #[clap(short, long)]
    url: Option<String>,
}

pub fn query(opts: QueryOpts) -> Result<(), Error> {
    todo!()
}
