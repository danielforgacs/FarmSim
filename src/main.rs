use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(25)
        .y_label_area_size(25)
        .build_cartesian_2d(0_f32..250_f32, 0_f32..100_f32)?;

    chart
        .configure_mesh()
        .draw()?;

    for i in 0..=5 {
        chart
            .draw_series(LineSeries::new(
                (0..=250).map(|x| x as f32).map(|x| (x + (i as f32), x )),
                &GREEN,
            ))?;
    }

    Ok(())
}
