use std::fs;
use eframe::egui;

pub fn load_ppm(path: &str) -> Result<(egui::ColorImage, (usize, usize)), String> {
    let file = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let mut lines = file.lines().filter(|l| !l.starts_with('#'));

    lines.next(); // Skip "P3"

    let mut size = lines
        .next()
        .ok_or("Missing size line")?
        .split_whitespace();

    let width: usize = size
        .next()
        .ok_or("Missing Width")?
        .parse()
        .map_err(|_| "Invalid width")?;

    let height: usize = size
        .next()
        .ok_or("Missing Height")?
        .parse()
        .map_err(|_| "Invalid height")?;

    let _max_val: usize = lines
        .next()
        .ok_or("Missing Max Value")?
        .parse()
        .map_err(|_| "Invalid Max Value")?;

    let mut pixarr = Vec::with_capacity(width * height * 3);
    for token in lines.flat_map(|l| l.split_whitespace()) {
        let val: u8 = token
            .parse()
            .map_err(|_| format!("Invalid Pixel value {}", token))?;
        pixarr.push(val);
    }

    if pixarr.len() != width * height * 3 {
        return Err(format!(
            "Pixel data size mismatch: expected {}, got {}",
            width * height * 3,
            pixarr.len()
        ));
    }

    let mut image = Vec::with_capacity(width * height * 4);
    for cnk in pixarr.chunks_exact(3) {
        image.extend_from_slice(&[cnk[0], cnk[1], cnk[2], 255]);
    }

    Ok((
        egui::ColorImage::from_rgba_unmultiplied([width, height], &image),
        (width, height),
    ))
}
