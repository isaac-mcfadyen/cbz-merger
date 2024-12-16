use std::{fs::File, path::PathBuf, time::Duration};

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

#[derive(Debug, Parser)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(num_args = 1..)]
    pub files: Vec<PathBuf>,

    #[clap(short = 'o', long)]
    pub output: PathBuf,
}

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let args = Args::parse();
    if args.output.extension().unwrap_or_default() != "cbz" {
        tracing::error!(
            "Output file must have a .cbz extension to be correctly interpreted by e-readers."
        );
        std::process::exit(1);
    }

    // Start spinner.
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    spinner.set_message("Extracted 0 images.");

    // Unzip all images to tempdir.
    let mut progress = 0;
    let mut extracted_names = Vec::new();
    let tempdir = tempfile::tempdir()?;
    for file in args.files {
        let zip_file = File::open(&file)?;
        let mut zip = ZipArchive::new(zip_file)?;
        for i in 0..zip.len() {
            let mut img_file = zip.by_index(i)?;

            let name = img_file.name().to_string();
            extracted_names.push(name.clone());

            let output_path = tempdir.path().join(&name);
            let mut output_file = File::create(&output_path)?;
            std::io::copy(&mut img_file, &mut output_file)?;

            progress += 1;
            spinner.set_message(format!("Extracted {} images.", progress));
        }
    }
    spinner.finish_and_clear();
    tracing::info!("Successfully extracted {} images.", progress);

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());
    spinner.set_message("Wrote 0 images.");

    let mut progress = 0;
    let output_file = File::create(&args.output)?;
    let mut output = ZipWriter::new(output_file);
    for name in extracted_names {
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::DEFLATE);
        let mut f = File::open(tempdir.path().join(&name))?;

        // Write the file.
        output.start_file(name, options)?;
        std::io::copy(&mut f, &mut output)?;

        progress += 1;
        spinner.set_message(format!("Wrote {} images.", progress));
    }

    spinner.finish_and_clear();
    tracing::info!("Successfully wrote {} images.", progress);

    output.finish()?;
    drop(tempdir);

    tracing::info!("Successfully wrote merged CBZ file to {:?}", &args.output);
    Ok(())
}
