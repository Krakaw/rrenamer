mod error;
mod lookup;
mod files;

use dotenv;
use clap::{App, Arg};
use crate::files::input_file::InputFile;
use crate::lookup::tmdb::Tmdb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenv::dotenv().ok();

    let matches = App::new("Rrenamer")
        .version("0.0.1")
        .author("Krakaw")
        .about("Lookup and rename movie files")
        .arg(
            Arg::new("rename")
                .short('r')
                .long("rename")
                .value_name("rename")
                .multiple(true)
                .about("Files to rename")
                .takes_value(true),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .about("Show what would happen")
        )
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .about("Sets the level of verbosity"))
        .get_matches();


    let verbosity = matches.occurrences_of("v");
    // You can check the value provided by positional arguments, or option arguments
    if let Some(values) = matches.values_of("rename") {

        let dry_run = matches.is_present("dry-run");
        for value in values {
            if verbosity > 0 {
                println!("Checking file: {}", value);
            }
            let mut input_file = InputFile::new(value)?;
            let (search, year, _ext) = input_file.lookup_parts()?;
            input_file.set_movie_name( Tmdb::lookup(search, year).await?);
            println!("Rename {} to {}", input_file.input_path.display(), input_file.output_path()?.display());
            if !dry_run {
                input_file.rename_file()?;
            }

        }
    }

    Ok(())
}
