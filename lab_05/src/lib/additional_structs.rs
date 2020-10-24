#[derive(Default, Debug)]
pub struct RabinKarpTaskResult {
    pub data: StrPat,
    pub result: Vec<usize>,
}

#[derive(Default, Clone, Debug)]
pub struct StrPat {
    pub string: String,
    pub pattern: String,
}

impl StrPat {
    pub fn new(string: String, pattern: String) -> Self {
        Self {
            string,
            pattern,
        }
    }
}

