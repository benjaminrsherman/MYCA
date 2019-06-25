pub use catalog::course::*;
pub use catalog::*;
pub use schedule::*;

mod catalog {
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
        /// # use myca::catalog::Catalog;
        /// # use myca::catalog::course::{Course, CourseID};
        /// # use serde_json::json;
        /// # use serde::Deserialize;
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

            for coid_set in &course.prereqs {
                for coid in coid_set {
                    if let Some(found_course) = self.get_course_mut(&coid) {
                        found_course.add_postoption(&coid);
                    } else {
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
        /// # use myca::catalog::Catalog;
        /// # use myca::catalog::course::CourseID;
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
        /// # use myca::catalog::Catalog;
        /// # use myca::catalog::course::CourseID;
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
        /// # use myca::catalog::Catalog;
        /// # use myca::catalog::course::CourseID;
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

        use serde::Deserialize;

        /// Used to identify courses in the schedule and catalog.
        #[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
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
            /// # use myca::catalog::course::CourseID;
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
        /// # use myca::catalog::course::CourseID;
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
        #[derive(Deserialize)]
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
            /// # use myca::catalog::course::{Course, CourseID};
            /// # use serde_json::json;
            /// # use serde::Deserialize;
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
            /// # use myca::catalog::course::{Course, CourseID};
            /// # use std::collections::HashSet;
            /// # use serde_json::json;
            /// # use serde::Deserialize;
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
            /// # use myca::catalog::course::{Course, CourseID};
            /// # use std::collections::HashSet;
            /// # use serde_json::json;
            /// # use serde::Deserialize;
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
        /// # use myca::catalog::course::{Course, CourseID};
        /// # use serde_json::json;
        /// # use serde::Deserialize;
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
}

mod schedule {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, HashSet};
    use std::fmt;

    use super::catalog::course::CourseID;
    use super::catalog::Catalog;

    /// Used to identify a semester in the schedule.
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub enum SemTime {
        Fall(i32),
        Spring(i32),
        Summer(i32),
    }

    /// Semesters are ordered based on the time they represent (earlier times
    /// are "less" than later times).
    impl Ord for SemTime {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    /// Semesters are ordered based on the time they represent (earlier times
    /// are "less" than later times).
    impl PartialOrd for SemTime {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self == other {
                return Some(Ordering::Equal);
            }

            let (self_year, self_season) = match self {
                SemTime::Spring(year) => (year, 0),
                SemTime::Summer(year) => (year, 1),
                SemTime::Fall(year) => (year, 2),
            };

            let (other_year, other_season) = match other {
                SemTime::Spring(year) => (year, 0),
                SemTime::Summer(year) => (year, 1),
                SemTime::Fall(year) => (year, 2),
            };

            if self_year != other_year {
                return self_year.partial_cmp(other_year);
            }

            self_season.partial_cmp(&other_season)
        }
    }

    /// This data structure stores the set of all courses for a given
    /// university semester.
    #[derive(Clone, Debug)]
    pub struct Semester {
        courses: HashSet<CourseID>,
        time: SemTime,
    }

    impl Semester {
        /// Generates a new semester corresponding to the given time.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::{Semester, SemTime};
        /// #
        /// let semester = Semester::new(SemTime::Fall(2019));
        /// ```
        pub fn new(time: SemTime) -> Self {
            Self {
                courses: HashSet::new(),
                time,
            }
        }

        /// Adds a course to the semester.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::{Semester, SemTime};
        /// # use myca::catalog::course::CourseID;
        /// #
        /// let mut semester = Semester::new(SemTime::Fall(2019));
        /// let coid = CourseID::new("TEST", 1100);
        ///
        /// semester.add_course(&coid);
        ///
        /// assert!(semester.contains(&coid));
        /// ```
        pub fn add_course(&mut self, coid: &CourseID) {
            self.courses.insert(coid.clone());
        }

        /// Removes a course from the semester.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::{Semester, SemTime};
        /// # use myca::catalog::course::CourseID;
        /// #
        /// let mut semester = Semester::new(SemTime::Fall(2019));
        /// let coid = CourseID::new("TEST", 1100);
        ///
        /// semester.add_course(&coid);
        ///
        /// assert!(semester.contains(&coid));
        ///
        /// semester.remove_course(&coid);
        ///
        /// assert!(!semester.contains(&coid));
        /// ```
        pub fn remove_course(&mut self, coid: &CourseID) {
            self.courses.remove(coid);
        }

        /// Returns if a course is in the semester.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::{Semester, SemTime};
        /// # use myca::catalog::course::CourseID;
        /// #
        /// let mut semester = Semester::new(SemTime::Fall(2019));
        /// let coid = CourseID::new("TEST", 1100);
        ///
        /// semester.add_course(&coid);
        ///
        /// assert!(semester.contains(&coid));
        /// ```
        pub fn contains(&self, coid: &CourseID) -> bool {
            self.courses.contains(coid)
        }

        /// Returns the SemTime corresponding to the semester.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::{Semester, SemTime};
        /// #
        /// let time = SemTime::Fall(2019);
        /// let semester = Semester::new(time.clone());
        ///
        /// assert_eq!(semester.get_time(), &time);
        /// ```
        pub fn get_time(&self) -> &SemTime {
            &self.time
        }
    }

    /// Formats semester for printing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::schedule::{Semester, SemTime};
    /// # use myca::catalog::course::CourseID;
    /// #
    /// let mut semester = Semester::new(SemTime::Fall(2019));
    /// semester.add_course(&CourseID::new("TEST", 1100));
    ///
    /// let semester_expected_output = String::from("Fall 2019:\n\tTEST 1100\n");
    ///
    /// let semester_output = format!("{}", semester);
    ///
    /// println!("{}", semester_expected_output);
    /// assert_eq!(semester_output, semester_expected_output);
    /// ```
    impl fmt::Display for Semester {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut output = String::new();

            match self.time {
                SemTime::Fall(year) => output = format!("{}Fall {}:\n", output, year),
                SemTime::Spring(year) => output = format!("{}Spring {}:\n", output, year),
                SemTime::Summer(year) => output = format!("{}Summer {}:\n", output, year),
            };

            for coid in &self.courses {
                output = format!("{}\t{}\n", output, coid);
            }

            write!(f, "{}", output)
        }
    }

    /// Stores one variant of a set of semesters.
    #[derive(Clone, Debug)]
    pub struct Schedule {
        semesters: BTreeMap<SemTime, Semester>,
    }

    impl Schedule {
        /// Generates a new schedule.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::Schedule;
        /// #
        /// let schedule = Schedule::new();
        /// ```
        pub fn new() -> Self {
            Self {
                semesters: BTreeMap::new(),
            }
        }

        /// Adds a semester to the schedule.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// #
        /// let mut schedule = Schedule::new();
        /// let semester = Semester::new(SemTime::Fall(2019));
        ///
        /// schedule.add_semester(semester.clone());
        /// ```
        pub fn add_semester(&mut self, sem: Semester) {
            self.semesters.insert(sem.time.clone(), sem);
        }

        /// Adds a course to the schedule at a given time.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// # use myca::catalog::course::CourseID;
        /// #
        /// let mut schedule = Schedule::new();
        /// let semester = Semester::new(SemTime::Fall(2019));
        ///
        /// schedule.add_semester(semester);
        ///
        /// assert!(schedule.add_course(&SemTime::Fall(2019), &CourseID::new("TEST", 1100)));
        /// ```
        pub fn add_course(&mut self, sem: &SemTime, coid: &CourseID) -> bool {
            match self.get_semester_mut(sem) {
                Some(sem) => {
                    sem.add_course(coid);
                    true
                }
                None => false,
            }
        }

        /// Returns a reference to the semester corresponding to the given
        /// SemTime, if it exists.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// #
        /// let mut schedule = Schedule::new();
        /// let semester = Semester::new(SemTime::Fall(2019));
        ///
        /// schedule.add_semester(semester);
        ///
        /// assert!(schedule.get_semester(&SemTime::Fall(2019)).is_some());
        /// ```
        pub fn get_semester(&self, sem: &SemTime) -> Option<&Semester> {
            self.semesters.get(sem)
        }

        /// Returns a mutable reference to the semester corresponding to the
        /// given SemTime, if it exists.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// #
        /// let mut schedule = Schedule::new();
        /// let semester = Semester::new(SemTime::Fall(2019));
        ///
        /// schedule.add_semester(semester);
        ///
        /// assert!(schedule.get_semester_mut(&SemTime::Fall(2019)).is_some());
        /// ```
        pub fn get_semester_mut(&mut self, sem: &SemTime) -> Option<&mut Semester> {
            self.semesters.get_mut(sem)
        }

        /// Removes a semester from the schedule.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// #
        /// let mut schedule = Schedule::new();
        /// let semester = Semester::new(SemTime::Fall(2019));
        ///
        /// schedule.add_semester(semester);
        ///
        /// assert!(schedule.get_semester(&SemTime::Fall(2019)).is_some());
        ///
        /// schedule.remove_semester(&SemTime::Fall(2019));
        ///
        /// assert!(schedule.get_semester(&SemTime::Fall(2019)).is_none());
        /// ```
        pub fn remove_semester(&mut self, sem: &SemTime) {
            self.semesters.remove(sem);
        }

        /// Returns a reference to the semesters in the schedule.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::schedule::*;
        /// #
        /// let mut schedule = Schedule::new();
        /// let sem1 = Semester::new(SemTime::Fall(2019));
        /// let sem2 = Semester::new(SemTime::Spring(2020));
        ///
        /// schedule.add_semester(sem1);
        /// schedule.add_semester(sem2);
        ///
        /// schedule.semesters();
        /// ```
        pub fn semesters(&self) -> &BTreeMap<SemTime, Semester> {
            &self.semesters
        }

        /// Returns if the given course can be found in the schedule.
        ///
        /// # Examples
        ///
        /// ```
        /// # use myca::catalog::course::CourseID;
        /// # use myca::schedule::*;
        /// #
        /// let coid = CourseID::new("TEST", 1100);
        /// let coid2 = CourseID::new("TEST", 1200);
        ///
        /// let mut semester = Semester::new(SemTime::Fall(2019));
        /// semester.add_course(&coid);
        ///
        /// let mut schedule = Schedule::new();
        /// schedule.add_semester(semester);
        ///
        /// assert!(schedule.contains(&coid));
        /// assert!(!schedule.contains(&coid2));
        /// ```
        pub fn contains(&self, coid: &CourseID) -> bool {
            for (_, semester) in &self.semesters {
                if semester.contains(coid) {
                    return true;
                }
            }

            false
        }

        fn try_add(&self, coid: &CourseID, sem: &SemTime, catalog: &Catalog) -> Option<Self> {
            if self.contains(coid) {
                return None;
            }

            let course = catalog.get_course(coid).unwrap();
            for prereq_set in course.prereq_sets() {
                let mut valid = false;

                for prereq in prereq_set {
                    if valid {
                        break;
                    }

                    for (time, semester) in &self.semesters {
                        if valid || time >= sem {
                            break;
                        }

                        if semester.contains(prereq) {
                            valid = true;
                        }
                    }
                }

                if !valid {
                    return None;
                }
            }

            let mut new_sem = self.clone();
            new_sem.add_course(sem, coid);
            Some(new_sem)
        }

        /// Generates all possible schedules which can be created by adding the
        /// given course into the schedule.
        ///
        /// Recursively adds prerequisites based on the catalog entry as well.
        pub fn add_course_to_schedule(
            coid: &CourseID,
            sched: &Schedule,
            catalog: &Catalog,
        ) -> Vec<Schedule> {
            let course = match catalog.get_course(coid) {
                Some(c) => c,
                None => return Vec::new(),
            };

            // Place prerequisites into schedule first
            let mut working_vec = vec![sched.clone()];
            for prereq_set in course.prereq_sets() {
                // curr_set will hold the set of all schedules from one of these prerequisites
                let mut curr_set = Vec::new();
                for prereq in prereq_set {
                    for schedule in &working_vec {
                        let mut prereq_options =
                            Self::add_course_to_schedule(prereq, &schedule, catalog);
                        curr_set.append(&mut prereq_options);
                    }
                }
                working_vec = curr_set;
            }

            // TODO: Handle corequisites

            let mut valid_schedules = Vec::new();
            for sched in working_vec {
                for (time, _) in sched.semesters() {
                    if let Some(new_sched) = sched.try_add(coid, time, catalog) {
                        valid_schedules.push(new_sched);
                    }
                }
            }

            valid_schedules
        }
    }

    /// Schedules are output by printing their semesters in chronological order.
    impl fmt::Display for Schedule {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut output = String::new();

            for (_, semester) in &self.semesters {
                output = format!("{}{}", output, semester);
            }

            write!(f, "{}", output)
        }
    }

    // TESTING FUNCTION
    pub fn get_schedules(coid: &CourseID, catalog: &Catalog) -> Vec<Schedule> {
        let mut base_sched = Schedule::new();
        base_sched.add_semester(Semester::new(SemTime::Fall(0)));
        base_sched.add_semester(Semester::new(SemTime::Spring(1)));

        base_sched.add_semester(Semester::new(SemTime::Fall(1)));
        base_sched.add_semester(Semester::new(SemTime::Spring(2)));

        base_sched.add_semester(Semester::new(SemTime::Summer(2)));
        base_sched.add_semester(Semester::new(SemTime::Fall(2)));

        base_sched.add_semester(Semester::new(SemTime::Fall(3)));
        base_sched.add_semester(Semester::new(SemTime::Spring(4)));

        Schedule::add_course_to_schedule(coid, &base_sched, catalog)
    }
}
