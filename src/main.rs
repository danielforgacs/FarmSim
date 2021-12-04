use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart
        .configure_mesh()
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?;
    chart
        .draw_series(LineSeries::new(
            (-150..=150).map(|x| (x as f32 + 0.2) / 150.0).map(|x| (x, x )),
            &GREEN,
        ))?;

    Ok(())
}
