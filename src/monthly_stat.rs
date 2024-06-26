use std::ops::Add;

#[derive(PartialEq, Debug)]
pub struct MonthlyStat {
    pub month: String,
    pub users: u32,
    pub lessons: u32,
    pub project_submissions: u32,
    pub projects_liked: u32,
}

impl Add for MonthlyStat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            month: self.month,
            users: self.users + rhs.users,
            lessons: self.lessons + rhs.lessons,
            project_submissions: self.project_submissions + rhs.project_submissions,
            projects_liked: self.projects_liked + rhs.projects_liked,
        }
    }
}

impl MonthlyStat {
    pub fn parse(str: &str) -> Self {
        let date: Vec<&str> = str
            .lines()
            .filter(|l| !l.is_empty())
            .take(1)
            .flat_map(|l| l.split(' '))
            .collect();

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

    #[test]
    fn parse_with_new_lines() {
        let expected = MonthlyStat {
            month: "June 1999".to_string(),
            users: 2020,
            lessons: 1010,
            project_submissions: 2,
            projects_liked: 300,
        };
        let str = "\r\nJune 22th, 1999\r\n2020 users signed up\r\n1010 lessons completed\r\n2 project submissions added\r\n300 projects liked\r\n";

        let result = MonthlyStat::parse(str);
        assert_eq!(expected, result);
    }

    #[test]
    fn adding_another_month() {
        let first = MonthlyStat {
            month: "December 2020".to_string(),
            users: 20,
            lessons: 50,
            project_submissions: 2,
            projects_liked: 300,
        };

        let second = MonthlyStat {
            month: "June 1999".to_string(),
            users: 100,
            lessons: 200,
            project_submissions: 3,
            projects_liked: 300,
        };

        let expect = MonthlyStat {
            month: "December 2020".to_string(),
            users: 120,
            lessons: 250,
            project_submissions: 5,
            projects_liked: 600,
        };

        assert_eq!(expect, first + second);
    }
}
