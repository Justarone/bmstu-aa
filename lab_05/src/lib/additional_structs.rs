use super::task::{ NUMBER_OF_MEASURMENTS };
use chrono::{ DateTime, Utc };

#[derive(Debug)]
pub struct RabinKarpTaskResult {
    pub data: StrPat,
    pub result: Vec<usize>,
    pub times: [DateTime<Utc>; NUMBER_OF_MEASURMENTS],
}

#[derive(Default, Clone, Debug)]
pub struct StrPat {
    pub string: Vec<char>,
    pub pattern: Vec<char>,
}

impl StrPat {
    pub fn new(string: Vec<char>, pattern: Vec<char>) -> Self {
        Self {
            string,
            pattern,
        }
    }
}

