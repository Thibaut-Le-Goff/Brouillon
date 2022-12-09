use plotters::prelude::*;

pub fn test(test_vec: Vec<(f32, f32)>, test_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    // send the result of the function in a result containing:
    //  - the result in case of succes = ()
    //  - a box (a pointer but its keeps ownership, in short)

     
    //let test_vec: Vec<(f32, f32)> = vec![(1.0, 3.3), (2., 2.1), (3., 1.5), (4., 1.9), (5., 1.0)];
   
    //let mut chart_builder = ChartBuilder::on(&drawing_area);

    let root = BitMapBackend::new(&test_str, (1280, 960)).into_drawing_area();
    root.fill(&WHITE);

    //determine the size between the chart and the end of the image
    let root = root.margin(10, 10, 10, 15);

    /* 
    let mut min_x: f32 = test_vec[0].0;
    let mut max_x: f32 = test_vec[0].0;
    let mut min_y: f32 = test_vec[0].1;
    let mut max_y: f32 = test_vec[0].1;

    for i in 0..= test_vec.len() - 1 {
        if test_vec[i].0 > max_x {
            max_x = test_vec[i].0;
        }
        if test_vec[i].0 < min_x {
            min_x = test_vec[i].0;
        }
        if test_vec[i].1 > max_y {
            max_y = test_vec[i].1;
        }
        if test_vec[i].1 < min_y {
            min_y = test_vec[i].1;
        }
    }
    */

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

        // test
        //.build_cartesian_2d(min_x + (max_x / 10.0)..max_x + (max_x / 10.0), min_y + (min_y / 10.0)..max_y + (max_y / 10.0))?;
        // to be more convinient to see, I want to create some space between the datas and the end of the chart
            //example:
            // -10 + (-10 / 2) = -15 => bon pour min si min est negatif, recule(min) par rapport à -10
            // -10 - (-10 / 2) = -5 => bon pour max si max est negatif, avence(max) par rapport à -10

            // 10 - (10 / 2) = 5 => bon pour min si min est positif, recule(min) par rapport à 10
            // 10 + (10 / 2) = 15 => bon pour max si max est positif, avence(max) par rapport à 10

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
    Ok(())
}

fn main() {
    let calc_test: f32 = 3 as f32;
    //let test_vec: Box<Vec<(f32, f32)>> = Box::new(vec![(calc_test + 1.0, calc_test / 3.3), (2.0 * calc_test, calc_test * 2.1), (3.0 - calc_test, 1.5 + calc_test), (calc_test - 4.0, 1.9 / calc_test), (5.0 - calc_test, 1.0 / calc_test)]);
    let test_vec: Vec<(f32, f32)> = vec![(calc_test + 1.0, calc_test / 3.3), (2.0 * calc_test, calc_test * 2.1), (3.0 - calc_test, 1.5 + calc_test), (calc_test - 4.0, 1.9 / calc_test), (5.0 - calc_test, 1.0 / calc_test)];

        
    let mut min_x: f32 = test_vec[0].0;
    let mut max_x: f32 = test_vec[0].0;
    let mut min_y: f32 = test_vec[0].1;
    let mut max_y: f32 = test_vec[0].1;
    
    for i in 0..= test_vec.len() - 1 {
        if test_vec[i].0 > max_x {
            max_x = test_vec[i].0;
        }
        if test_vec[i].0 < min_x {
            min_x = test_vec[i].0;
        }
        if test_vec[i].1 > max_y {
            max_y = test_vec[i].1;
        }
        if test_vec[i].1 < min_y {
            min_y = test_vec[i].1;
        }
    }

    for i in 0..= 2 {
        //let test_vec2: = &test_vec;
        // order : x, y
       
        /* 
    
        */  
       
        //let slope: f32 = 2;
        //let intercept: f32 = 10;
        let test_str: Box<String> = Box::new(format!("plotters-doc-data-test-{}.png", i+1));

        test((test_vec).to_vec(), &test_str).ok();
        //test(test_vec, &test_str).ok();
    }

}

////// Test concluant ////////
/*
fn main() {
    let test_vec: Vec<(f32, f32)> = vec![(1.0, -3.3), (2., 2.1), (-3., 1.5), (4., 1.9), (5., 1.0)];

    println!("{:?}", test_vec[1]);
    println!("{:?}", test_vec[1].0);
    
    let mut min_x: f32 = test_vec[0].0;
    let mut max_x: f32 = test_vec[0].0;
    let mut min_y: f32 = test_vec[0].1;
    let mut max_y: f32 = test_vec[0].1;

    for i in test_vec {
        if i.0 > max_x {
            max_x = i.0;
        }
        if i.0 < min_x {
            min_x = i.0;
        }
        if i.1 > max_y {
            max_y = i.1;
        }
        if i.1 < min_y {
            min_y = i.1;
        }
    }
    
    println!("L'axe des x va de {:?} à {:?}", min_x, max_x);
    println!("L'axe des y va de {:?} à {:?}", min_y, max_y);
}
*/