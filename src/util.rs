pub fn is_in_range(v: f64, min: f64, max: f64) -> bool {

    v.is_finite() && v >= min && v <= max

}

pub fn is_in_range_exclusive(v: f64, min: f64, max: f64) -> bool {

    v.is_finite() && v > min && v < max

}