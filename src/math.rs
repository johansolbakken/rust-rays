use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_double_between(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
