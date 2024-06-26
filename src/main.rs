use std::{
    fs::File,
    io::{BufReader, Read},
};

use charts::{
    AxisPosition, Chart, Color, LineSeriesView, PointLabelPosition, ScaleBand, ScaleLinear,
};
use indexmap::IndexMap;
use monthly_stat::MonthlyStat;

mod monthly_stat;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("top-stats")?;
    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf)?;

    let mut stats_map: IndexMap<String, MonthlyStat> = IndexMap::new();
    let stats = buf
        .split("==============")
        .map(MonthlyStat::parse)
        .collect::<Vec<MonthlyStat>>();

    stats.into_iter().for_each(|ns| {
        match stats_map.get_mut(&ns.month) {
            Some(s) => *s += ns,
            None => {
                stats_map.insert(ns.month.clone(), ns);
            }
        };
    });

    let vec = stats_map.into_values().collect::<Vec<MonthlyStat>>();

    draw_chart(&vec);
    lessons_chart(&vec);

    Ok(())
}

fn draw_chart(stats: &[MonthlyStat]) {
    let width = 1920;
    let height = 800;
    let (top, right, bottom, left) = (50, 0, 50, 80);

    let x = ScaleBand::new()
        .set_domain(
            stats
                .iter()
                .map(|s| s.month.clone())
                .collect::<Vec<String>>(),
        )
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 35000_f32])
        .set_range(vec![height - top - bottom, 0]);

    let mut users = vec![];
    let mut project_submissions = vec![];
    let mut projects_liked = vec![];

    stats.iter().for_each(|s| {
        users.push((s.month.clone(), s.users as f32));
        project_submissions.push((s.month.clone(), s.projects_liked as f32));
        projects_liked.push((s.month.clone(), s.project_submissions as f32));
    });

    let users_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_custom_data_label("User Sign Ups".to_string())
        .set_label_position(PointLabelPosition::N)
        .set_label_visibility(false)
        .load_data(&users)
        .unwrap();

    let project_submissions_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_custom_data_label("Project Submissions".to_string())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#aa0000"]))
        .set_label_visibility(false)
        .load_data(&project_submissions)
        .unwrap();

    let projects_liked_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_custom_data_label("Projects Liked".to_string())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#555555"]))
        .set_label_position(PointLabelPosition::N)
        .set_label_visibility(false)
        .load_data(&projects_liked)
        .unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("TOP Growth"))
        .add_view(&users_view)
        .add_view(&project_submissions_view)
        .add_view(&projects_liked_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_legend_at(AxisPosition::Bottom)
        .save("top-chart.svg")
        .unwrap();
}

fn lessons_chart(stats: &[MonthlyStat]) {
    let width = 1920;
    let height = 800;
    let (top, right, bottom, left) = (50, 0, 50, 80);

    let x = ScaleBand::new()
        .set_domain(
            stats
                .iter()
                .map(|s| s.month.clone())
                .collect::<Vec<String>>(),
        )
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![200000_f32, 400000_f32])
        .set_range(vec![height - top - bottom, 0]);

    let lessons = &stats
        .iter()
        .map(|s| (s.month.clone(), s.lessons as f32))
        .collect();

    let lessons_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .load_data(lessons)
        .unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("TOP Lessons Completed"))
        .add_view(&lessons_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .save("lessons-chart.svg")
        .unwrap();
}
