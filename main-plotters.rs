use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // send the result of the function in a result containing:
    //  - the result in case of succes = ()
    //  - a box (a pointer but its keeps ownership, in short)

     
    let test_vec: Vec<(f32, f32)> = vec![(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];
   
    //let mut chart_builder = ChartBuilder::on(&drawing_area);

    let root = BitMapBackend::new("plotters-doc-data.png", (1280, 960)).into_drawing_area();
    root.fill(&WHITE);

    //let mut chart_context = chart_builder.build_cartesian_2d(0.0..5.5, 0.0..5.5).unwrap();
    let root = root.margin(10, 10, 10, 15);

    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("Gradient descent", ("sans-serif", 40))//.into_font())
        // Set the size of the label region
        // the size of the between values
        .x_label_area_size(20)
        .y_label_area_size(40)
        // length of values of the x and y axis
        .build_cartesian_2d(-10f32..10f32, -20f32..10f32)?;

    // Then we can draw a mesh
    // configuration du quadrillage
    chart.configure_mesh()
        .max_light_lines(5)
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        //&GREEN
        .max_light_lines(5)

        .y_labels(10)
        //.max_light_lines(5)
        // We can also change the format of the label text
        // show number of zero after the dot
        //.y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // And we can draw something in the drawing area
    // put the red line 
    //let test_vec: Vec<(f64, f64)> = vec![(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];
    chart.draw_series(LineSeries::new(
        vec![(0.4, 0.2), (5.1, 5.2999), (8.38363, 7.298727)],
        //chart_context.draw_series(LineSeries::new(data, BLACK)).unwrap()
        &GREEN,
    ))?;
    // Similarly, we can draw point series
    // put the red dot
    chart.draw_series(PointSeries::of_element(
        //vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0), test_vec[1]],
        test_vec,
        5,
        &BLACK,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
        },

    ))?;
    root.present()?;
    
    /*
    //use plotters::prelude::*;
    let data = [(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];
    let drawing_area = SVGBackend::new("configure_series_labels.svg", (300, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder.margin(7).set_left_and_bottom_label_area_size(20);
    let mut chart_context = chart_builder.build_cartesian_2d(-5.0..5.5, -5.0..5.5).unwrap();
    chart_context.configure_mesh().draw().unwrap();
    chart_context.draw_series(LineSeries::new(data, BLACK)).unwrap().label("Series 1")
        .legend(|(x,y)| Rectangle::new([(x - 15, y + 1), (x, y)], BLACK));
    chart_context.configure_series_labels().position(SeriesLabelPosition::UpperRight).margin(20)
        .legend_area_size(5).border_style(BLUE).background_style(BLUE.mix(0.1)).label_font(("Calibri", 20)).draw().unwrap();
    */

    Ok(())
}