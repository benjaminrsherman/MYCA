use std::collections::{HashSet, HashMap};
use serde::Deserialize;
use std::fmt;

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

    pub fn from(coid: &str) -> Option<CourseID> {
        let char_vec: Vec<char> = coid.chars().collect();

        if char_vec.len() != 8 && char_vec.len() != 9 {
            return None;
        }

        let code_str: String = char_vec[char_vec.len()-4..].iter().collect();

        let code = match code_str.parse::<u16>(){
            Ok(num) => num,
            _ => return None
        };

        for character in &char_vec[..4] {
            if character < &'A' || character > &'Z' {
                return None;
            }
        }

        let subj: String = char_vec[..4].iter().collect();

        Some(CourseID { subj, code })
    }
}

impl fmt::Display for CourseID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.subj, self.code)
    }
}

#[derive(Deserialize)]
pub struct Course {
    name: String,
    description: String,
    coid: CourseID,
    prereqs: Vec<CourseID>,
    post_options: HashSet<CourseID>,
}

impl Course {
    pub fn new(coid: &CourseID) -> Self {
        Course {
            name: String::new(),
            description: String::new(),
            coid: coid.clone(),
            prereqs: Vec::new(),
            post_options: HashSet::new(),
        }
    }

    pub fn get_id(&self) -> CourseID {
        self.coid.clone()
    }

    pub fn add_prereq(&mut self, coid: &CourseID) {
        self.prereqs.push(coid.clone());
    }

    fn add_postoption(&mut self, coid: &CourseID) {
        self.post_options.insert(coid.clone());
    }
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.coid, self.name)
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
        if let Some(existing_course) = self.get_course(&course.get_id()) {
            course.post_options = existing_course.post_options.clone();
            self.courses.remove(&course.get_id());
        }

        for coid in &course.prereqs {
            if let Some(found_course) = self.get_course_mut(&coid) {
                found_course.add_postoption(&coid);
            } else {
                let mut new_course = Course::new(&coid);
                new_course.add_postoption(&coid);
                self.courses.insert(coid.clone(), new_course);
            }
        }

        self.courses.insert(course.get_id(), course);
    }

    pub fn emplace_course(&mut self, coid: &CourseID) {
        self.add_course(Course::new(coid));
    }

    pub fn get_course(&self, coid: &CourseID) -> Option<&Course> {
        self.courses.get(coid)
    }

    pub fn get_course_mut(&mut self, coid: &CourseID) -> Option<&mut Course> {
        self.courses.get_mut(coid)
    }

    pub fn get_course_tree(&self, coid: &CourseID) -> Option<Vec<&Course>> {
        let course = match self.get_course(coid) {
            Some(c) => c,
            None => return None
        };

        let mut course_tree = Vec::new();

        for coid in &course.prereqs {
            let mut prereq_tree = self.get_course_tree(coid).unwrap_or_else(|| panic!("Error: Malformed catalog!"));
            course_tree.append(&mut prereq_tree);
        }

        course_tree.push(course);

        Some(course_tree)
    }
}
