use anyhow::Result;
use ndarray::prelude::*;
use plotters::prelude::*;
use plotters::style::full_palette::GREY_200;

pub fn draw(output: &str, sim: &Array2<u8>) -> Result<()> {
    let root = BitMapBackend::new(output, (8192, 4096)).into_drawing_area();

    root.fill(&WHITE)?;

    let sub_areas = root.split_evenly((sim.shape()[0], sim.shape()[1]));

    let cells: Vec<u8> = sim.flatten().to_vec();

    for (i, area) in sub_areas.iter().enumerate() {
        if cells[i] == 1 {
            area.margin(1, 1, 1, 1).fill(&BLACK)?;
        } else {
            area.margin(1, 1, 1, 1).fill(&GREY_200)?;
        }
    }

    root.present().expect("Unable to write result to file");
    println!("Result has been saved to {}", output);

    Ok(())
}
