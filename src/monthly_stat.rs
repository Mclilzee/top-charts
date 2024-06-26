pub struct MonthlyStat {
    pub month: String,
    pub users: u32,
    pub lessons: u32,
    pub project_submissions: u32,
    pub projects_liked: u32,
}

impl MonthlyStat {
    fn parse(str: &str) -> Self {
        Self {
            month: "December 2022".to_string(),
            users: 400,
            lessons: 200,
            project_submissions: 1000,
            projects_liked: 202020,
        }
    }
}

#[cfg(test)]
mod test {
    use super::MonthlyStat;

    #[test]
    fn parse_stat() {
        let expected = MonthlyStat {
            month: "December 2022".to_string(),
            users: 400,
            lessons: 200,
            project_submissions: 1000,
            projects_liked: 202020,
        };
    }
}
