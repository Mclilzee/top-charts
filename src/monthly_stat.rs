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
        let date: Vec<&str> = str.lines().take(1).flat_map(|l| l.split(' ')).collect();
        let month = format!("{} {}", date.first().unwrap(), date.get(2).unwrap());

        let nums = str
            .lines()
            .skip(1)
            .filter_map(|l| l.split(' ').nth(0))
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<u32>>();

        Self {
            month,
            users: *nums.first().unwrap(),
            lessons: *nums.get(1).unwrap(),
            project_submissions: *nums.get(2).unwrap(),
            projects_liked: *nums.get(3).unwrap(),
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

    #[test]
    fn parse_another_stat() {
        let expected = MonthlyStat {
            month: "June 1999".to_string(),
            users: 2020,
            lessons: 1010,
            project_submissions: 2,
            projects_liked: 300,
        };

        let str = r#"June 22th, 1999
2020 users signed up
1010 lessons completed
2 project submissions added
300 projects liked"#;

        let result = MonthlyStat::parse(str);
        assert_eq!(expected, result);
    }
}
