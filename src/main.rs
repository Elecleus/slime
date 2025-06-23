use anyhow::Result;

mod args;
mod test;
mod process;

use process::*;

fn main() -> Result<()> {
    let my_args = args::get_args()?;

    match my_args.images {
        Some(_) => from_images(my_args)?,
        None => from_pipe(my_args)?,
    }
    
    Ok(())
}
