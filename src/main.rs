use std::fmt;

struct Data {
    values: Vec<f64>,
}

impl Data {
    fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    fn variance(&self) -> f64 {
        let mean = self.mean();
        let mut sum_of_squared_diffs = 0.0;
        for value in &self.values {
            let diff = value - mean;
            sum_of_squared_diffs += diff * diff;
        }
        sum_of_squared_diffs / (self.values.len() as f64 - 1.0)
    }

    fn mean(&self) -> f64 {
        let sum: f64 = self.values.iter().sum();
        sum / (self.values.len() as f64)
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}

fn main() {
    let data = Data::new(vec![1.0, 2.0, 4.0, 6.0, 3.0]);
    println!("Data: {}", data);
    let variance = data.variance();
    println!("Variance: {}", variance);
}