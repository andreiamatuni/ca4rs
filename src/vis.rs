use anyhow::Result;
use plotters::coord::Shift;
use plotters::prelude::*;
use plotters::style::full_palette::{GREY_200, GREY_50, RED_100};

pub fn format_sim(sim: Vec<Vec<u8>>) {
    for row in sim.iter() {
        println!("{:?}", row);
    }
}

pub fn draw(output: &str, sim: &Vec<Vec<u8>>) -> Result<()> {
    let root = BitMapBackend::new(output, (1024, 1024)).into_drawing_area();

    root.fill(&WHITE)?;

    let root = root.titled("CA output", ("sans-serif", 60))?;
    // .shrink(((1024 - 700) / 2, 0), (700, 700));

    let sub_areas = root.split_evenly((sim.len(), sim[0].len()));

    let cells: Vec<u8> = sim.into_iter().flatten().map(|i| i.to_owned()).collect();

    for (i, area) in sub_areas.iter().enumerate() {
        if cells[i] == 1 {
            area.margin(2, 2, 2, 2).fill(&BLACK)?;
        } else {
            // area.fill(&BLACK)?;
            area.margin(2, 2, 2, 2).fill(&GREY_200)?;
        }
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", output);

    Ok(())
}
