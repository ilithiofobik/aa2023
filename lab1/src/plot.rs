use plotters::prelude::*;

pub fn plot_histogram (data: &Vec<(usize, usize)>, n: usize, u: Option<usize>, num_of_exps: usize) -> Result<(), Box<dyn std::error::Error>> {
    let upper_x = data.iter().map(|(x, _)| *x).max().unwrap() as u32 + 1;
    let upper_y = data.iter().map(|(_, y)| *y).max().unwrap() as u32 + 1;

    let caption = 
        match u {
            Some(u) => format!("Histogram of {} experiments with n={} and u={}", num_of_exps, n, u),
            None => format!("Histogram of {} experiments with n={}", num_of_exps, n)
        };
    let file_name = 
        match u {
            Some(u) => format!("../plots/histogram_n{}_u{}.png", n, u),
            None => format!("../plots/histogram_n{}.png", n)
        };

    let root = BitMapBackend::new(&file_name, (4000, 2000)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(caption, ("sans-serif", 50.0))
        .build_cartesian_2d((1..upper_x).into_segmented(), 0..upper_y)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Num of slots")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|(x, y)| (*x as u32, *y as u32))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", file_name);

    Ok(())
}