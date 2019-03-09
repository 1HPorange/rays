mod vec3;

use vec3::*;

struct Ray {
    origin: Vec3;
    direction: Vec3;
}

struct RayHitInfo {


}

trait RayTarget {

    Option<RayHitInfo> test_intersection()

}