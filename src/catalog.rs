use std::collections::HashMap;

use course::*;

/// Stores all courses offered by a university
#[derive(Default)]
pub struct Catalog {
    courses: HashMap<CourseID, Course>,
}

impl Catalog {
    /// Generate a new catalog
    pub fn new() -> Self {
        Catalog {
            courses: HashMap::new(),
        }
    }

    /// Adds a new course to the catalog.  The course is added to the
    /// post_options list of every one of its prerequisites.
    ///
    /// # Warning:
    /// If the course already exists, it will be overwritten except for
    /// its post_options.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::{Catalog, Course, CourseID};
    /// # use serde_json::json;
    /// # use serde::{Deserialize, Serialize};
    /// #
    /// let mut catalog = Catalog::new();
    /// let coid = CourseID::new("TEST", 1100);
    /// let course: Course = serde_json::from_value(json!({
    /// # "complete": true,
    /// # "name": "",
    /// # "description": "",
    /// # "offered": "",
    /// # "age_reqs": "",
    /// # "prereqs": [],
    /// # "prereqs_opt": [],
    /// # "coreqs": [],
    /// # "coreqs_opt": [],
    /// # "post_options": [],
    ///     "coid": {
    ///         "subj": "TEST",
    ///         "code": 1100
    ///     }
    /// })).unwrap();
    ///
    /// catalog.add_course(course);
    ///
    /// assert!(catalog.get_course(&coid).is_some());
    /// ```
    pub fn add_course(&mut self, mut course: Course) {
        if let Some(existing_course) = self.get_course(&course.get_id()) {
            course.post_options = existing_course.post_options.clone();
            self.courses.remove(&course.get_id());
        }

        for coid in course.prereqs.iter().flatten() {
            match self.get_course_mut(coid) {
                Some(found_course) => found_course.add_postoption(&coid),
                None => {
                    let mut new_course = Course::new(&coid);
                    new_course.add_postoption(&coid);
                    self.courses.insert(coid.clone(), new_course);
                }
            }
        }

        self.courses.insert(course.get_id(), course);
    }

    /// Generates a new course in place given a course ID.  As the course
    /// is made only from its ID, it will have no prerequisites or
    /// associated information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::{Catalog, CourseID};
    /// #
    /// let mut catalog = Catalog::new();
    /// let coid = CourseID::new("TEST", 1100);
    ///
    /// catalog.emplace_course(&coid);
    ///
    /// assert!(catalog.get_course(&coid).is_some());
    /// ```
    pub fn emplace_course(&mut self, coid: &CourseID) {
        self.add_course(Course::new(coid));
    }

    /// Returns a reference to the course if it exists in the catalog, or
    /// `None` if it is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::{Catalog, CourseID};
    /// #
    /// let mut catalog = Catalog::new();
    /// let coid = CourseID::new("TEST", 1100);
    ///
    /// catalog.emplace_course(&coid);
    ///
    /// let course = catalog.get_course(&coid).unwrap();
    ///
    /// assert_eq!(course.get_id(), coid);
    /// ```
    pub fn get_course(&self, coid: &CourseID) -> Option<&Course> {
        self.courses.get(coid)
    }

    /// Returns a mutable reference to a course if it exists in the
    /// catalog, or `None` if it is not found.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::{Catalog, CourseID};
    /// #
    /// let mut catalog = Catalog::new();
    /// let coid = CourseID::new("TEST", 1200);
    ///
    /// catalog.emplace_course(&coid);
    ///
    /// let course = catalog.get_course_mut(&coid).unwrap();
    ///
    /// assert_eq!(course.get_id(), coid);
    ///
    /// course.add_prereq(&CourseID::new("TEST", 1100));
    /// ```
    pub fn get_course_mut(&mut self, coid: &CourseID) -> Option<&mut Course> {
        self.courses.get_mut(coid)
    }
}

pub(crate) mod course {
    use std::collections::HashSet;
    use std::fmt;

    use serde::{Deserialize, Serialize};

    /// Used to identify courses in the schedule and catalog.
    #[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Debug)]
    pub struct CourseID {
        subj: String,
        code: u16,
    }

    impl CourseID {
        /// Constructs a new course from a given subject and course code.
        pub fn new(subj: &str, code: u16) -> Self {
            CourseID {
                subj: String::from(subj),
                code,
            }
        }

        /// Parses a course ID from a string.  The string must be 8 or 9
        /// characters long.  The first four characters are parsed as the
        /// course's subject and the last four characters are parsed
        /// as the course's code.
        /// Returns the parsed CourseID if input string is valid.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::CourseID;
        /// #
        /// let coid = CourseID::from("TEST 1100").unwrap();
        /// assert_eq!(coid, CourseID::new("TEST", 1100));
        /// ```
        pub fn from(coid: &str) -> Option<CourseID> {
            let char_vec: Vec<char> = coid.chars().collect();

            if char_vec.len() != 8 && char_vec.len() != 9 {
                return None;
            }

            let code_str: String = char_vec[char_vec.len() - 4..].iter().collect();

            let code = match code_str.parse::<u16>() {
                Ok(num) => num,
                _ => return None,
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

    /// CourseIDs are printed in the format: "SUBJ CODE"
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::CourseID;
    /// #
    /// let coid = CourseID::new("TEST", 1100);
    /// let coid_str = format!("{}", coid);
    ///
    /// assert_eq!(coid_str, "TEST 1100");
    /// ```
    impl fmt::Display for CourseID {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} {}", self.subj, self.code)
        }
    }

    /// Stores all information related to a course.
    ///
    /// This should only be generated when parsed from a json file.
    #[derive(Deserialize, Serialize)]
    #[allow(dead_code)] // TODO: use all of the fields
    pub struct Course {
        pub(super) complete: bool,

        pub(super) name: String,
        pub(super) description: String,

        pub(super) coid: CourseID,

        pub(super) offered: String,
        pub(super) age_reqs: String,

        pub(super) prereqs: Vec<HashSet<CourseID>>,
        pub(super) prereqs_opt: HashSet<CourseID>,

        pub(super) coreqs: Vec<HashSet<CourseID>>,
        pub(super) coreqs_opt: HashSet<CourseID>,

        pub(super) post_options: HashSet<CourseID>,
    }

    impl Course {
        pub(super) fn new(coid: &CourseID) -> Self {
            Course {
                complete: true,
                name: String::new(),
                description: String::new(),
                coid: coid.clone(),
                offered: String::new(),
                age_reqs: String::new(),
                prereqs: Vec::new(),
                prereqs_opt: HashSet::new(),
                coreqs: Vec::new(),
                coreqs_opt: HashSet::new(),
                post_options: HashSet::new(),
            }
        }

        /// Returns the CourseID corresponding to this course.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::{Course, CourseID};
        /// # use serde_json::json;
        /// # use serde::{Deserialize, Serialize};
        /// #
        /// let course: Course = serde_json::from_value(json!({
        /// # "complete": true,
        /// # "name": "",
        /// # "description": "",
        /// # "offered": "",
        /// # "age_reqs": "",
        /// # "prereqs": [],
        /// # "prereqs_opt": [],
        /// # "coreqs": [],
        /// # "coreqs_opt": [],
        /// # "post_options": [],
        ///     "coid": {
        ///         "subj": "TEST",
        ///         "code": 1100
        ///     }
        /// })).unwrap();
        ///
        /// assert_eq!(course.get_id(), CourseID::new("TEST", 1100));
        /// ```
        pub fn get_id(&self) -> CourseID {
            self.coid.clone()
        }

        /// Adds a prerequisite to the course.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::{Course, CourseID};
        /// # use std::collections::HashSet;
        /// # use serde_json::json;
        /// # use serde::{Deserialize, Serialize};
        /// #
        /// let mut course: Course = serde_json::from_value(json!({
        /// # "complete": true,
        /// # "name": "",
        /// # "description": "",
        /// # "offered": "",
        /// # "age_reqs": "",
        /// # "prereqs": [],
        /// # "prereqs_opt": [],
        /// # "coreqs": [],
        /// # "coreqs_opt": [],
        /// # "post_options": [],
        ///     "coid": {
        ///         "subj": "TEST",
        ///         "code": 1200
        ///     }
        /// })).unwrap();
        ///
        /// course.add_prereq(&CourseID::new("TEST", 1100));
        ///
        /// let mut prereq_sets = Vec::new();
        /// prereq_sets.push(HashSet::new());
        /// prereq_sets[0].insert(CourseID::new("TEST", 1100));
        ///
        /// assert_eq!(course.prereq_sets(), &prereq_sets);
        /// ```
        pub fn add_prereq(&mut self, coid: &CourseID) {
            let mut hashset = HashSet::new();
            hashset.insert(coid.clone());
            self.prereqs.push(hashset);
        }

        pub(super) fn add_postoption(&mut self, coid: &CourseID) {
            self.post_options.insert(coid.clone());
        }

        /// Returns the corequisites for a course.  For each HashSet
        /// in the returned vector, only one course is required.
        pub fn coreq_sets(&self) -> &Vec<HashSet<CourseID>> {
            &self.coreqs
        }

        /// Returns the prerequisites for the course.  For each HashSet
        /// in the returned vector, only one course is required.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::{Course, CourseID};
        /// # use std::collections::HashSet;
        /// # use serde_json::json;
        /// # use serde::{Deserialize, Serialize};
        /// #
        /// let mut course: Course = serde_json::from_value(json!({
        /// # "complete": true,
        /// # "name": "",
        /// # "description": "",
        /// # "offered": "",
        /// # "age_reqs": "",
        /// # "prereqs": [],
        /// # "prereqs_opt": [],
        /// # "coreqs": [],
        /// # "coreqs_opt": [],
        /// # "post_options": [],
        ///     "coid": {
        ///         "subj": "TEST",
        ///         "code": 1200
        ///     }
        /// })).unwrap();
        ///
        /// course.add_prereq(&CourseID::new("TEST", 1100));
        ///
        /// let mut prereq_sets = Vec::new();
        /// prereq_sets.push(HashSet::new());
        /// prereq_sets[0].insert(CourseID::new("TEST", 1100));
        ///
        /// assert_eq!(course.prereq_sets(), &prereq_sets);
        /// ```
        pub fn prereq_sets(&self) -> &Vec<HashSet<CourseID>> {
            &self.prereqs
        }
    }

    /// Courses are printed in the following format: "COID: NAME"
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::{Course, CourseID};
    /// # use serde_json::json;
    /// # use serde::{Deserialize, Serialize};
    /// #
    /// let course: Course = serde_json::from_value(json!({
    /// # "complete": true,
    /// # "name": "",
    /// # "description": "",
    /// # "offered": "",
    /// # "age_reqs": "",
    /// # "prereqs": [],
    /// # "prereqs_opt": [],
    /// # "coreqs": [],
    /// # "coreqs_opt": [],
    /// # "post_options": [],
    ///     "coid": {
    ///         "subj": "TEST",
    ///         "code": 1100
    ///     },
    ///     "name": "Introduction to Testing"
    /// })).unwrap();
    ///
    /// let course_str = format!("{}", course);
    ///
    /// assert_eq!(course_str, "TEST 1100: Introduction to Testing");
    /// ```
    impl fmt::Display for Course {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {}", self.coid, self.name)
        }
    }
}
