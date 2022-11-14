use batch_edit_file::OpenDir;
use clap::Parser;
use log::info;
use std::io::Result;

fn main() -> Result<()> {
    let mut opt = OpenDir::parse();

    opt.edit_files_name()?;

    info!("edit file name success!");

    Ok(())
}
