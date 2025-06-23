use crate::args::MyArgs;
use anyhow::Result;
use opencv::highgui;
use opencv::imgcodecs::{ImreadModes, imread};
use opencv::prelude::*;

#[derive(Debug)]
pub enum Direction {
    Down,
    Up,
    Left,
    Right,
}

pub fn from_images(my_args: MyArgs) -> Result<()> {
    let imgs = my_args
        .images
        .unwrap()
        .iter()
        .map(|name| {
            imread(name, ImreadModes::IMREAD_COLOR_BGR.into()).map_err(|e| anyhow::Error::new(e))
        })
        .collect::<Result<Vec<Mat>>>()?;

    let final_img = imgs
        .into_iter()
        .reduce(|acc, e| combine_two_images_down(acc, e.clone()).unwrap_or(e))
        .unwrap();

    loop {
        highgui::imshow("test", &final_img)?;

        let key = highgui::wait_key(10)?;
        if key == 255 {
            break;
        }
    }

    Ok(())
}

pub fn from_pipe(my_args: MyArgs) -> Result<()> {
    // [TODO] from_pipe
    Ok(())
}

// [TODO] write a macro

fn combine_two_images_down(a: Mat, b: Mat) -> Result<Mat> {
    let a_rows = a.rows();

    let b_firstline = b.row_bounds(0, 1)?;
    let mut least_row_diff: f64 = 99999999.0; // A big value at first loop.
    let mut least_row_index: i32 = 0;
    for i in 0..a_rows {
        let a_to_diff_row = a.row_bounds(i, i + 1)?;

        let mut diff_output = Mat::default();
        opencv::core::absdiff(&a_to_diff_row, &b_firstline, &mut diff_output)?;
        diff_output = diff_output.mul(&diff_output, 1.0)?.to_mat()?;
        let diff_val = opencv::core::mean(&diff_output, &opencv::core::no_array())?[0];

        if diff_val < least_row_diff {
            least_row_diff = diff_val;
            least_row_index = i;
        }
    }

    let mut result: Mat = Mat::default();

    opencv::core::vconcat2(&a.row_bounds(0, least_row_index)?, &b, &mut result)?;

    Ok(result)
}

fn combine_two_images_up(a: &Mat, b: &Mat) -> Result<()> {
    Ok(())
}

fn combine_two_images_left(a: &Mat, b: &Mat) -> Result<()> {
    Ok(())
}

fn combine_two_images_right(a: &Mat, b: &Mat) -> Result<()> {
    Ok(())
}
