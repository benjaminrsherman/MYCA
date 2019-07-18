use std::path::PathBuf;
use structopt::StructOpt;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

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

pub fn load_schedules(schedule_fname: &Option<PathBuf>) -> Vec<Schedule> {
    match schedule_fname {
        None => vec![Schedule::new()],
        Some(schedule_fname) => {
            let file = match File::open(schedule_fname) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error opening current schedule: {:?}", err);
                    eprintln!("Defaulting to an empty schedule...");
                    return vec![Schedule::new()];
                }
            };

            let buf_reader = BufReader::new(file);
            match serde_json::from_reader(buf_reader) {
                Ok(schedule) => schedule,
                Err(err) => {
                    eprintln!("Error parsing the current schedule: {:?}", err);
                    eprintln!("Defaulting to an empty schedule");
                    vec![Schedule::new()]
                }
            }
        }
    }
}

/// CLI Options
#[derive(StructOpt, Debug)]
#[structopt(name = "rpi_planner")]
struct Opt {
    /// Catalog file
    #[structopt(short = "c", long = "catalog", parse(from_os_str))]
    catalog: PathBuf,

    /// Pre-existing schedule file
    #[structopt(short = "s", long = "current_schedule", parse(from_os_str))]
    schedule: Option<PathBuf>,

    /// Output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,

    /// Courses to add to schedule
    #[structopt(name = "COURSE")]
    courses: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    let catalog_fname = opt.catalog.to_str().unwrap();
    let catalog =
        parse_catalog(catalog_fname).unwrap_or_else(|err| panic!("Error parsing catalog: {}", err));

    let mut schedules = load_schedules(&opt.schedule);

    for input_coid in opt.courses {
        let coid = match CourseID::from(&input_coid) {
            Some(id) => id,
            None => {
                eprintln!("Error: '{}' is not in the Course ID format", input_coid);
                continue;
            }
        };
        schedules = get_schedules(&coid, &catalog, schedules);

        println!("Found {} schedule(s) for {}:", schedules.len(), coid);
        for (i, schedule) in schedules.iter().enumerate() {
            println!("Schedule {}:", i + 1);
            println!("{}", schedule);
        }
    }

    if let Some(output_file) = opt.output {
        let file = File::open(output_file).expect("Unable to open output file");

        let buf_writer = BufWriter::new(file);
        serde_json::to_writer(buf_writer, &schedules).expect("Error writing to output file");
    }
}
