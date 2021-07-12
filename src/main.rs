mod error;
mod lookup;
mod files;

use dotenv;
use clap::{App, Arg};
use crate::files::input_file::InputFile;
use crate::lookup::tmdb::Tmdb;
use crate::lookup::tmdb_results::TmdbResult;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .arg(
            Arg::new("prompt")
                .short('p')
                .long("prompt")
                .about("Show options and prompt for input")
        )
        .arg(Arg::new("v")
            .short('v')
            .multiple_occurrences(true)
            .about("Sets the level of verbosity"))
        .get_matches();

    let verbosity = matches.occurrences_of("v");
    let prompt = matches.is_present("prompt");
    // You can check the value provided by positional arguments, or option arguments
    if let Some(values) = matches.values_of("rename") {
        let dry_run = matches.is_present("dry-run");
        for value in values {
            if verbosity > 0 {
                println!("Checking file: {}", value);
            }
            let mut input_file = InputFile::new(value)?;
            let (search, year, _ext) = input_file.lookup_parts()?;
            let name_results = Tmdb::lookup(search, year).await?;
            let name_result;
            if prompt {
                println!("Choose a result for {}:\n{}", value, name_results);
                let mut input = String::new();
                name_result = match std::io::stdin().read_line(&mut input) {
                    Ok(_input) => {
                        let index: usize = input.trim().parse().unwrap();
                        name_results.0.get(index).map(|s: &TmdbResult| s.to_string())
                    }
                    Err(_no_update) => {
                        None
                    }
                }
            } else {
                name_result = name_results.0.first().map(|s| s.to_string())
            }

            if let Some(name_result) = name_result {
                input_file.set_movie_name(name_result);
                println!("Rename {} to {}", input_file.input_path.display(), input_file.output_path()?.display());
                if !dry_run {
                    input_file.rename_file()?;
                }
            }
        }
    }

    Ok(())
}
