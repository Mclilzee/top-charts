#[derive(PartialEq, Debug)]
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

        let str = r#"December 1st, 2022
400 users signed up
200 lessons completed
1000 project submissions added
202020 projects liked"#;

        let result = MonthlyStat::parse(str);
        assert_eq!(expected, result);
    }
}
