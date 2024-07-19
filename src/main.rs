extern crate nalgebra_glm as glm;

use glm::Vec3;

fn main() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    println!("v1: ({}, {}, {})", v1.x, v1.y, v1.z);
    println!("v2: ({}, {}, {})", v2.x, v2.y, v2.z);

    let sum = v1 + v2;
    let diff = v1 - v2;
    let scaled = v1 * 2.0;

    println!("sum: ({}, {}, {})", sum.x, sum.y, sum.z);
    println!("diff: ({}, {}, {})", diff.x, diff.y, diff.z);
    println!("scaled: ({}, {}, {})", scaled.x, scaled.y, scaled.z);

    let dot_product = v1.dot(&v2);
    println!("dot product: {}", dot_product);

    let cross_product = v1.cross(&v2);
    // println!("cross product: {}", cross_product);
    println!("cross_product: ({}, {}, {})", cross_product.x, cross_product.y, cross_product.z);

    let magnitude = v1.magnitude();
    println!("magnitude: {}", magnitude);

    let normalized = v1.normalize();
    println!("normalized: ({}, {}, {})", normalized.x, normalized.y, normalized.z);

    let magnitude_normalized = normalized.magnitude();
    println!("magnitude normalized: {}", magnitude_normalized);
} 
