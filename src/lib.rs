use plotters::prelude::*;

pub fn draw_graphics(coordinates: Vec<(f32, f32)>, length: f32, height: f32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("/Users/ivrez/sandbox/rust/projectile_programm/graph.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let min_x;
    let max_x;
    let max_y;

    if length < 0f32 {
        min_x = length - 10f32;
        max_x = 10f32;
        //max_y = height + 10f32;
        max_y = 100f32;
    }
    else {
        min_x = 0f32;
        max_x = length + 10f32;
        //max_y = height + 10f32;
        max_y = 100f32;
    }

    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("now", ("sans-serif", 30).into_font())
        // Set the size of the label region
        .x_label_area_size(40)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(min_x..max_x, 0f32..max_y)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        .draw()?;

    // Draw lines
    chart.draw_series(LineSeries::new(
        //vec![(0.0, 0.0), (20.76, 18.68), (41.53, 33.18), (62.29, 43.51), (83.05, 49.67), (103.81, 51.66), (124.58, 49.47), (145.34, 43.11), (166.10, 32.58), (186.87, 17.88), (207.63, -1.00)],
        coordinates.clone(),
        &RED,
    ))?;

    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        //vec![(0.0, 0.0), (20.76, 18.68), (41.53, 33.18), (62.29, 43.51), (83.05, 49.67), (103.81, 51.66), (124.58, 49.47), (145.34, 43.11), (166.10, 32.58), (186.87, 17.88), (207.63, -1.00)],
        coordinates.clone(),
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 12).into_font());
        },
    ))?;

    Ok(())
}
