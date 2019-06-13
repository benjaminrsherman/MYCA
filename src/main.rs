use rpi_planner::parse_catalog;

use std::path::PathBuf;
use structopt::StructOpt;

/// CLI Options
#[derive(StructOpt, Debug)]
#[structopt(name = "rpi_planner")]
struct Opt {
    /// Catalog file
    #[structopt(short = "c", long = "catalog", parse(from_os_str))]
    catalog: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let catalog_fname = opt.catalog.to_str().unwrap();
    let catalog = parse_catalog(catalog_fname)
        .unwrap_or_else(|err| panic!("Error parsing catalog: {}", err));
}
