use std::env;
use ppm_viewer::app::App;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ppmv <file.ppm>");
        std::process::exit(1);
    }

    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ppmv",
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(args[1].clone())))),
    )
}