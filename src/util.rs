pub fn is_in_range<T: num_traits::Float>(v: T, min: T, max: T) -> bool {

    v.is_finite() && v >= min && v <= max

}