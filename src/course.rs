use std::collections::{HashSet, HashMap};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct CourseID {
    subj: String,
    code: u16,
}

impl CourseID {
    pub fn new(subj: &str, code: u16) -> Self {
        CourseID {
            subj: String::from(subj),
            code
        }
    }
}

#[derive(Deserialize)]
pub struct Course {
    name: String,
    description: String,
    id: CourseID,
    prereqs: Vec<CourseID>,
    post_options: HashSet<CourseID>,
}

impl Course {
    pub fn new(id: &CourseID) -> Self {
        Course {
            name: String::new(),
            description: String::new(),
            id: id.clone(),
            prereqs: Vec::new(),
            post_options: HashSet::new(),
        }
    }

    pub fn getID(&self) -> CourseID {
        self.id.clone()
    }

    pub fn add_prereq(&mut self, id: &CourseID) {
        self.prereqs.push(id.clone());
    }

    fn add_postoption(&mut self, id: &CourseID) {
        self.post_options.insert(id.clone());
    }
}

pub struct Catalog {
    courses: HashMap<CourseID, Course>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { courses: HashMap::new() }
    }

    pub fn add_course(&mut self, mut course: Course) {
        if let Some(existing_course) = self.get_course(&course.getID()) {
            course.post_options = existing_course.post_options.clone();
            self.courses.remove(&course.getID());
        }

        for id in &course.prereqs {
            if let Some(found_course) = self.get_course_mut(&id) {
                found_course.add_postoption(&id);
            } else {
                let mut new_course = Course::new(&id);
                new_course.add_postoption(&id);
                self.courses.insert(id.clone(), new_course);
            }
        }

        self.courses.insert(course.getID(), course);
    }

    pub fn emplace_course(&mut self, id: &CourseID) {
        self.add_course(Course::new(id));
    }

    pub fn get_course(&self, id: &CourseID) -> Option<&Course> {
        self.courses.get(id)
    }

    pub fn get_course_mut(&mut self, id: &CourseID) -> Option<&mut Course> {
        self.courses.get_mut(id)
    }
}
