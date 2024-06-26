use std::{
    fs::File,
    io::{BufReader, Read},
};

use charts::{AxisPosition, Chart, Color, LineSeriesView, MarkerType, ScaleBand, ScaleLinear};
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

    draw_chart(&stats_map.into_values().collect::<Vec<MonthlyStat>>());
    Ok(())
}

fn draw_chart(stats: &[MonthlyStat]) {
    let first_month = &stats.first().unwrap().month;
    let last_month = &stats.last().unwrap().month;

    let width = 1920;
    let height = 800;
    let (top, right, bottom, left) = (50, 0, 50, 80);

    // Create a band scale that will interpolate values in [0, 200] to values in the
    // [0, availableWidth] range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(
            stats
                .iter()
                .map(|s| s.month.clone())
                .collect::<Vec<String>>(),
        )
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 400000_f32])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let mut users = vec![];
    let mut lessons = vec![];
    let mut project_submissions = vec![];
    let mut projects_liked = vec![];

    stats.iter().for_each(|s| {
        users.push((s.month.clone(), s.users as f32));
        lessons.push((s.month.clone(), s.lessons as f32));
        project_submissions.push((s.month.clone(), s.projects_liked as f32));
        projects_liked.push((s.month.clone(), s.project_submissions as f32));
    });

    let users_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_custom_data_label("User Sign Ups".to_string())
        .set_label_visibility(false)
        .load_data(&users)
        .unwrap();

    let lessons_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_custom_data_label("Lessons Completed".to_string())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#aaaa00"]))
        .set_label_visibility(false)
        .load_data(&lessons)
        .unwrap();

    let project_submissions_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_custom_data_label("Project Submissions".to_string())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#aa0000"]))
        .load_data(&project_submissions)
        .unwrap();

    let projects_liked_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_custom_data_label("Projects Liked".to_string())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#555555"]))
        .load_data(&projects_liked)
        .unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("TOP Growth"))
        .add_view(&users_view)
        .add_view(&lessons_view)
        .add_view(&project_submissions_view)
        .add_view(&projects_liked_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_bottom_axis_label(format!("{} - {}", first_month, last_month))
        .add_legend_at(AxisPosition::Right)
        .save("line-chart.svg")
        .unwrap();
}
