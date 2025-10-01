use ppm_viewer::app::App;
use std::env;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ppmv <file.ppm>");
        std::process::exit(1);
    }

    let options = eframe::NativeOptions::default();
    let path = args[1].clone();
    eframe::run_native(
        &path.clone(),
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(path.clone())))),
    )
}
