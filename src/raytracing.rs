use super::vec3::*;

struct Ray<T> {
    origin: Vec3<T>,
    direction: Vec3Norm<T>
}

struct RayHitInfo {


}

trait RayTarget {

    fn test_intersection<T>(ray: &Ray<T>) -> Option<RayHitInfo>;

}