use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("plotters-doc-data/twoscale.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(5)
        .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
        .build_ranged(0f32..10f32, LogRange(0.1f32..1e10f32))?
        .set_secondary_coord(0f32..10f32, -1.0f32..1.0f32);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .y_desc("Log Scale")
        .y_label_formatter(&|x| format!("{:e}", x))
        .draw()?;

    chart
        .configure_secondary_axes()
        .y_desc("Linear Scale")
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|x| (x as f32 / 10.0, (1.02f32).powf(x as f32 * x as f32 / 10.0))),
            &BLUE,
        ))?
        .label("y = 1.02^x^2")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .draw_secondary_series(LineSeries::new(
            (0..=100).map(|x| (x as f32 / 10.0, (x as f32 / 5.0).sin())),
            &RED,
        ))?
        .label("y = sin(2x)")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&RGBColor(128, 128, 128))
        .draw()?;

    Ok(())
}
