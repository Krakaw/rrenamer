mod error;
mod lookup;
mod files;

use dotenv;
use clap::{App, Arg};
use std::path::Path;
use crate::files::input_file::InputFile;
use crate::lookup::tmdb::Tmdb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    let matches = App::new("Rrenamer")
        .version("0.0.1")
        .author("Krakaw")
        .about("Lookup and rename")
        .arg(
            Arg::new("rename")
                .short('r')
                .long("rename")
                .value_name("rename")
                .multiple(true)
                .about("Files to rename")
                .takes_value(true),
        )
        .get_matches();


    // You can check the value provided by positional arguments, or option arguments
    if let Some(values) = matches.values_of("rename") {
        for value in values {
            let mut input_file = InputFile::new(value)?;
            let (search, year, ext) = input_file.lookup_parts()?;
            input_file.set_movie_name( Tmdb::lookup(search, year).await?);
            input_file.rename_file();
        }
    }

    Ok(())
}
