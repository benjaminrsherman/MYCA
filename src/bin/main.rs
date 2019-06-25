use std::path::PathBuf;
use structopt::StructOpt;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

extern crate myca;
use myca::*;

#[derive(Deserialize)]
struct Courses {
    courses: Vec<Course>,
}

pub fn parse_catalog(fname: &str) -> Result<Catalog, Box<dyn Error>> {
    let file = File::open(fname)?;

    let buf_reader = BufReader::new(file);
    let courses: Courses = serde_json::from_reader(buf_reader)?;

    let mut catalog = Catalog::new();

    for course in courses.courses {
        catalog.add_course(course);
    }

    Ok(catalog)
}

/// CLI Options
#[derive(StructOpt, Debug)]
#[structopt(name = "rpi_planner")]
struct Opt {
    /// Catalog file
    #[structopt(short = "c", long = "catalog", parse(from_os_str))]
    catalog: PathBuf,

    /// Courses to generate prerequisites for
    #[structopt(name = "COURSE")]
    courses: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let catalog_fname = opt.catalog.to_str().unwrap();
    let catalog =
        parse_catalog(catalog_fname).unwrap_or_else(|err| panic!("Error parsing catalog: {}", err));

    for input_coid in opt.courses {
        let coid = match CourseID::from(&input_coid) {
            Some(id) => id,
            None => {
                eprintln!("Error: '{}' is not in the Course ID format", input_coid);
                continue;
            }
        };
        let schedules = get_schedules(&coid, &catalog);

        println!("Found {} schedule(s) for {}:", schedules.len(), coid);
        for (i, schedule) in schedules.iter().enumerate() {
            println!("Schedule {}:", i + 1);
            println!("{}", schedule);
        }
    }
}
