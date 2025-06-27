use pyo3::{Python, PyResult, Py, wrap_pyfunction, Bound, prelude::{pyfunction, pymodule}, types::{PyModule, PyBytes}}; use image::{ImageBuffer, RgbaImage, open};

#[pyfunction]
fn remove_colour(path: &str, input_r: u8, input_g: u8, input_b: u8, sensitivity: u8) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    for y in 0..height { for x in 0..width {
        let [r, g, b, _] = image.get_pixel(x, y).0;
        if (input_r.saturating_sub(sensitivity) <= r && r <= input_r.saturating_add(sensitivity)) && (input_g.saturating_sub(sensitivity) <= g && g <= input_g.saturating_add(sensitivity)) && (input_b.saturating_sub(sensitivity) <= b && b <= input_b.saturating_add(sensitivity)) {image.put_pixel(x, y, image::Rgba([255, 69, 86, 0]));}
    }}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn flip_horizontally(path: &str) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    for y in 0..height { for x in 0..(width / 2) {
        let pixel_1 = image.get_pixel(x, y).clone(); let pixel_2 = image.get_pixel(width - x - 1, y).clone();
        image.put_pixel(x, y, pixel_2); image.put_pixel(width - x - 1, y, pixel_1)
    }}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn flip_vertically(path: &str) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    for x in 0..width { for y in 0..(height / 2) {
        let pixel_1 = image.get_pixel(x, y).clone(); let pixel_2 = image.get_pixel(x, height - y - 1).clone();
        image.put_pixel(x, y, pixel_2); image.put_pixel(x, height - y - 1, pixel_1)
    }}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn gray_scale(path: &str) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    for y in 0..height { for x in 0..width {
        let [r, g, b, a] = image.get_pixel(x, y).0; let gray = ((r as u16 + g as u16 + b as u16) / 3) as u8;
        image.put_pixel(x, y, image::Rgba([gray, gray, gray, a]))
    }}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn perceptual_gray_scale(path: &str) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    for y in 0..height { for x in 0..width {
        let [r, g, b, a] = image.get_pixel(x, y).0; let gray = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        image.put_pixel(x, y, image::Rgba([gray, gray, gray, a]))
    }}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn rotate_left(path: &str) -> PyResult<Py<PyBytes>> {
    let image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    let mut columns = vec![]; let mut column = vec![];
    for x in (0..width).rev() {for y in 0..height {column.push(*image.get_pixel(x, y))} columns.push(column); column = Vec::new()}
    let mut image: RgbaImage = ImageBuffer::new(height, width);
    for y in 0..width {for x in 0..height {image.put_pixel(x, y, columns[y as usize][x as usize])}}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn rotate_right(path: &str) -> PyResult<Py<PyBytes>> {
    let image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    let mut columns = vec![]; let mut column = vec![];
    for x in 0..width {for y in 0..height {column.push(*image.get_pixel(x, y))} columns.push(column); column = Vec::new()}
    let mut image: RgbaImage = ImageBuffer::new(height, width);
    for y in 0..width {for x in (0..height).rev() {image.put_pixel(x, y, columns[y as usize][x as usize])}}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn contrast(path: &str, contrast: u8, negative_contrast: bool, channels: String) -> PyResult<Py<PyBytes>> {
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions(); let channels = channels.to_lowercase();
    if channels.contains("r") && channels.contains("g") && channels.contains("b") {
        for y in 0..height { for x in 0..width {
        let [r, g, b, a] = image.get_pixel(x, y).0;
        let (r, g, b) = if negative_contrast {if (r as u16 + g as u16 + b as u16) / 3 <= 128 {(r.saturating_add(contrast), g.saturating_add(contrast), b.saturating_add(contrast))} else {(r.saturating_sub(contrast), g.saturating_sub(contrast), b.saturating_sub(contrast))}} else {if (r as u16 + g as u16 + b as u16) / 3 <= 128 {(r.saturating_sub(contrast), g.saturating_sub(contrast), b.saturating_sub(contrast))} else {(r.saturating_add(contrast), g.saturating_add(contrast), b.saturating_add(contrast))}};
        image.put_pixel(x, y, image::Rgba([r, g, b, a]))
    }}}
    else {
    if channels.contains("r") {
        for y in 0..height { for x in 0..width {
            let [mut r, g, b, a] = image.get_pixel(x, y).0;
            r = if negative_contrast {if r <= 128 {r.saturating_add(contrast)} else {r.saturating_sub(contrast)}} else {if r <= 128 {r.saturating_sub(contrast)} else {r.saturating_add(contrast)}};
            image.put_pixel(x, y, image::Rgba([r, g, b, a]))
    }}}
    if channels.contains("g") {
        for y in 0..height { for x in 0..width {
            let [r, mut g, b, a] = image.get_pixel(x, y).0;
            g = if negative_contrast {if g <= 128 {g.saturating_add(contrast)} else {g.saturating_sub(contrast)}} else {if g <= 128 {g.saturating_sub(contrast)} else {g.saturating_add(contrast)}};
            image.put_pixel(x, y, image::Rgba([r, g, b, a]))
    }}}
    if channels.contains("b") {
        for y in 0..height { for x in 0..width {
            let [r, g, mut b, a] = image.get_pixel(x, y).0;
            b = if negative_contrast {if b <= 128 {b.saturating_add(contrast)} else {b.saturating_sub(contrast)}} else {if b <= 128 {b.saturating_sub(contrast)} else {b.saturating_add(contrast)}};
            image.put_pixel(x, y, image::Rgba([r, g, b, a]))
    }}}
    }
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pyfunction]
fn random_pixels(width: u32, height: u32) -> PyResult<Py<PyBytes>> {
    let mut image: RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height { for x in 0..width {image.put_pixel(x, y, image::Rgba([fastrand::u8(0..=255), fastrand::u8(0..=255), fastrand::u8(0..=255), fastrand::u8(0..=255)]))}}
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pymodule]
fn hueclipse(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {m.add_function(wrap_pyfunction!(remove_colour, m)?)?; m.add_function(wrap_pyfunction!(flip_horizontally, m)?)?; m.add_function(wrap_pyfunction!(flip_vertically, m)?)?; m.add_function(wrap_pyfunction!(gray_scale, m)?)?; m.add_function(wrap_pyfunction!(perceptual_gray_scale, m)?)?; m.add_function(wrap_pyfunction!(rotate_left, m)?)?; m.add_function(wrap_pyfunction!(rotate_right, m)?)?; m.add_function(wrap_pyfunction!(contrast, m)?)?; m.add_function(wrap_pyfunction!(random_pixels, m)?)?; Ok(())}