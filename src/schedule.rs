use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::fmt;

use super::catalog::course::CourseID;
use super::catalog::Catalog;

/// Used to identify a semester in the schedule.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    /// # use myca::{Semester, SemTime};
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
    /// # use myca::{CourseID, Semester, SemTime};
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
    /// # use myca::{CourseID, Semester, SemTime};
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
    /// # use myca::{CourseID, Semester, SemTime};
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
    /// # use myca::{Semester, SemTime};
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
/// # use myca::{CourseID, Semester, SemTime};
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
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule {
    semesters: BTreeMap<SemTime, Semester>,
}

impl Schedule {
    /// Generates a new schedule.
    ///
    /// # Examples
    ///
    /// ```
    /// # use myca::Schedule;
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
    /// # use myca::{Schedule, Semester, SemTime};
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
    /// # use myca::{CourseID, Schedule, Semester, SemTime};
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
    /// # use myca::{Schedule, Semester, SemTime};
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
    /// # use myca::{Schedule, Semester, SemTime};
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
    /// # use myca::{Schedule, Semester, SemTime};
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
    /// # use myca::{Schedule, Semester, SemTime};
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
    /// # use myca::{CourseID, Schedule, Semester, SemTime};
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
        self.get_time(coid).is_some()
    }

    /// Returns the first semester the given course can be found in within
    /// the schedule, if such a course exists.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use myca::{CourseID, Schedule, Semester, SemTime};
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
    /// assert_eq!(schedule.get_time(&coid), Some(&SemTime::Fall(2019)));
    /// assert!(schedule.get_time(&coid2).is_none());
    /// ```
    pub fn get_time(&self, coid: &CourseID) -> Option<&SemTime> {
        for (time, semester) in &self.semesters {
            if semester.contains(coid) {
                return Some(time);
            }
        }

        None
    }

    fn try_add(&self, coid: &CourseID, sem: &SemTime, catalog: &Catalog) -> Option<Self> {
        if self.contains(coid) {
            return Some(self.clone());
        }

        let course = catalog.get_course(coid).unwrap();

        for coreq_set in course.coreq_sets() {
            let mut contains_at_least_one = false;
            let mut valid_time = false;
            for coreq in coreq_set {
                match self.get_time(coreq) {
                    Some(time) => {
                        contains_at_least_one = true;
                        if time == sem {
                            valid_time = true;
                            break;
                        }
                    }
                    None => continue,
                };
            }

            if contains_at_least_one != valid_time {
                return None;
            }
        }

        for prereq_set in course.prereq_sets() {
            if !prereq_set.iter().any(|prereq| {
                self.semesters
                    .iter()
                    .filter(|(time, _)| time < &sem)
                    .any(|(_, semester)| semester.contains(prereq))
            }) {
                return None;
            }
        }

        let mut new_sched = self.clone();
        new_sched.add_course(sem, coid);
        Some(new_sched)
    }

    /// Generates all possible schedules which can be created by adding the
    /// given course into the schedule.
    ///
    /// Recursively adds prerequisites and corequisites based on the catalog
    /// entry.
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
        let mut prereq_scheds = vec![sched.clone()];
        for prereq_set in course.prereq_sets() {
            // curr_set will hold the set of all schedules from one of these prerequisites
            let mut curr_set = Vec::new();
            for prereq in prereq_set {
                for schedule in &prereq_scheds {
                    if !schedule.contains(prereq) {
                        let mut prereq_options =
                            Self::add_course_to_schedule(prereq, &schedule, catalog);
                        curr_set.append(&mut prereq_options);
                    } else {
                        curr_set.push(schedule.clone());
                    }
                }
            }
            prereq_scheds = curr_set;
        }

        // Add this course to the schedule
        let mut prereq_and_this_scheds = Vec::new();
        for sched in prereq_scheds {
            for (time, _) in sched.semesters() {
                if let Some(new_sched) = sched.try_add(coid, time, catalog) {
                    prereq_and_this_scheds.push(new_sched);
                }
            }
        }

        // Add all corequisite courses to the schedule
        let mut all_scheds = prereq_and_this_scheds;
        for coreq_set in course.coreq_sets() {
            // curr_set will hold the set of all schedules from one of these corequisites
            let mut curr_set = Vec::new();
            for coreq in coreq_set {
                for schedule in &all_scheds {
                    if !schedule.contains(coreq) {
                        let mut coreq_options =
                            Self::add_course_to_schedule(coreq, &schedule, catalog);
                        curr_set.append(&mut coreq_options);
                    } else {
                        curr_set.push(schedule.clone());
                    }
                }
            }
            all_scheds = curr_set;
        }

        all_scheds
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
pub fn get_schedules(
    coid: &CourseID,
    catalog: &Catalog,
    schedules: Vec<Schedule>,
) -> Vec<Schedule> {
    schedules
        .iter()
        .map(|schedule| Schedule::add_course_to_schedule(coid, &schedule, catalog))
        .flatten()
        .collect()
}
