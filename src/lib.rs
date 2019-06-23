pub mod catalog {
    use std::collections::HashMap;

    use course::*;

    pub struct Catalog {
        courses: HashMap<CourseID, Course>,
    }

    impl Catalog {
        pub fn new() -> Self {
            Catalog {
                courses: HashMap::new(),
            }
        }

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

        pub fn emplace_course(&mut self, coid: &CourseID) {
            self.add_course(Course::new(coid));
        }

        pub fn get_course(&self, coid: &CourseID) -> Option<&Course> {
            self.courses.get(coid)
        }

        pub fn get_course_mut(&mut self, coid: &CourseID) -> Option<&mut Course> {
            self.courses.get_mut(coid)
        }
    }

    pub mod course {
        use std::collections::HashSet;
        use std::fmt;

        use serde::Deserialize;

        #[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
        pub struct CourseID {
            subj: String,
            code: u16,
        }

        impl CourseID {
            pub fn new(subj: &str, code: u16) -> Self {
                CourseID {
                    subj: String::from(subj),
                    code,
                }
            }

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

        impl fmt::Display for CourseID {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {}", self.subj, self.code)
            }
        }

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
            pub fn new(coid: &CourseID) -> Self {
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

            pub fn get_id(&self) -> CourseID {
                self.coid.clone()
            }

            pub fn add_prereq(&mut self, coid: &CourseID) {
                let mut hashset = HashSet::new();
                hashset.insert(coid.clone());
                self.prereqs.push(hashset);
            }

            pub(super) fn add_postoption(&mut self, coid: &CourseID) {
                self.post_options.insert(coid.clone());
            }

            pub fn coreq_sets(&self) -> &Vec<HashSet<CourseID>> {
                &self.coreqs
            }

            pub fn prereq_sets(&self) -> &Vec<HashSet<CourseID>> {
                &self.prereqs
            }
        }

        impl fmt::Display for Course {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}: {}", self.coid, self.name)
            }
        }
    }
}

pub mod schedule {
    use std::cmp::Ordering;
    use std::collections::{BTreeMap, HashSet};
    use std::fmt;

    use super::catalog::course::CourseID;
    use super::catalog::Catalog;

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub enum SemTime {
        Fall(i32),
        Spring(i32),
        Summer(i32),
    }

    impl Ord for SemTime {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

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

    #[derive(Clone, Debug)]
    pub struct Semester {
        courses: HashSet<CourseID>,
        time: SemTime,
    }

    impl Semester {
        pub fn new(time: SemTime) -> Self {
            Self {
                courses: HashSet::new(),
                time,
            }
        }

        pub fn add_course(&mut self, coid: &CourseID) {
            self.courses.insert(coid.clone());
        }

        pub fn remove_course(&mut self, coid: &CourseID) {
            self.courses.remove(coid);
        }

        pub fn contains(&self, coid: &CourseID) -> bool {
            self.courses.contains(coid)
        }
    }

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

    #[derive(Clone, Debug)]
    pub struct Schedule {
        semesters: BTreeMap<SemTime, Semester>,
    }

    impl Schedule {
        pub fn new() -> Self {
            Self {
                semesters: BTreeMap::new(),
            }
        }

        pub fn add_semester(&mut self, sem: Semester) {
            self.semesters.insert(sem.time.clone(), sem);
        }

        pub fn add_course(&mut self, sem: &SemTime, coid: &CourseID) -> bool {
            match self.get_semester_mut(sem) {
                Some(sem) => {
                    sem.add_course(coid);
                    true
                }
                None => false,
            }
        }

        pub fn get_semester(&self, sem: &SemTime) -> Option<&Semester> {
            self.semesters.get(sem)
        }

        pub fn get_semester_mut(&mut self, sem: &SemTime) -> Option<&mut Semester> {
            self.semesters.get_mut(sem)
        }

        pub fn remove_semester(&mut self, sem: &SemTime) {
            self.semesters.remove(sem);
        }

        pub fn semesters(&self) -> &BTreeMap<SemTime, Semester> {
            &self.semesters
        }

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
    }

    impl fmt::Display for Schedule {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut output = String::new();

            for (_, semester) in &self.semesters {
                output = format!("{}{}", output, semester);
            }

            write!(f, "{}", output)
        }
    }

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
                    let mut prereq_options = add_course_to_schedule(prereq, &schedule, catalog);
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

        add_course_to_schedule(coid, &base_sched, catalog)
    }
}
