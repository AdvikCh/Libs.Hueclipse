use pyo3::Python; use pyo3::PyResult; use pyo3::prelude::pyfunction; use pyo3::prelude::pymodule; use pyo3::types::PyModule; use pyo3::types::PyBytes; use pyo3::Py; use pyo3::wrap_pyfunction; use image::open;

#[pyfunction]
fn remove_colour(path: &str, input_r: u8, input_g: u8, input_b: u8, sensitivity: u8) -> PyResult<Py<PyBytes>> {
    let (mut x, mut y) = (0, 0);
    let mut image = open(path).unwrap().into_rgba8(); let (width, height) = image.dimensions();
    while x < width + 1 {
        let [r, g, b, _] = image.get_pixel(x, y).0;
        if (input_r.saturating_sub(sensitivity) <= r && r <= input_r.saturating_add(sensitivity)) && (input_g.saturating_sub(sensitivity) <= g && g <= input_g.saturating_add(sensitivity)) && (input_b.saturating_sub(sensitivity) <= b && b <= input_b.saturating_add(sensitivity)) {image.put_pixel(x, y, image::Rgba([255, 69, 86, 0]));}
        x += 1; if x == width {x = 0; y += 1;}
        if y == height {break;}
    }
    let buf = image.into_raw(); Python::with_gil(|py| Ok(PyBytes::new(py, &buf).into()))
}

#[pymodule]
fn hueclipse(_py: Python, m: &PyModule) -> PyResult<()> {m.add_function(wrap_pyfunction!(remove_colour, m)?)?; Ok(())}