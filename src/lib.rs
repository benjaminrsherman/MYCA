use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use serde::Deserialize;

mod course;
use crate::course::*;

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
