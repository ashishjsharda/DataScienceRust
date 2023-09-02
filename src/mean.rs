use ndarray::Array1;

pub(crate) fn mean(values: &Array1<f64>) -> f64 {
    values.mean().unwrap()
}