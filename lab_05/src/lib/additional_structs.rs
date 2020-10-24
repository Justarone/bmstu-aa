#[derive(Default, Debug)]
pub struct RabinKarpTaskResult {
    pub data: StrPat,
    pub result: Vec<usize>,
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

