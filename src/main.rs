const SOURCE_CODE: &str = "cube()";

fn facet_from_vertexes(v1: (f64, f64, f64), v2: (f64, f64, f64), v3: (f64, f64, f64)) {
    let u12 = (v2.0 - v1.0, v2.1 - v1.1, v2.2 - v1.2);
    let u13 = (v3.0 - v1.0, v3.1 - v1.1, v3.2 - v1.2);
    let normal = (
        u12.1 * u13.2 - u12.2 * u13.1,
        u12.2 * u13.0 - u12.0 * u13.2,
        u12.0 * u13.1 - u12.1 * u13.0,
    ); // Cross product
    let length = (normal.0.powi(2) + normal.1.powi(2) + normal.2.powi(2)).sqrt();
    let normal = (normal.0 / length, normal.1 / length, normal.2 / length);
    println!("facet normal {} {} {}", normal.0, normal.1, normal.2);
    println!("  outer loop");
    println!("    vertex {} {} {}", v1.0, v1.1, v1.2);
    println!("    vertex {} {} {}", v2.0, v2.1, v2.2);
    println!("    vertex {} {} {}", v3.0, v3.1, v3.2);
    println!("  endloop");
    println!("endfacet");
}

fn main() {
    if SOURCE_CODE == "cube()" {
        println!("solid made_by_orthoptera");
        // Z-axis Back
        facet_from_vertexes((0.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 0.0, 0.0));
        facet_from_vertexes((0.0, 1.0, 0.0), (1.0, 1.0, 0.0), (0.0, 0.0, 0.0));
        // Z-axis Front
        facet_from_vertexes((0.0, 0.0, 1.0), (1.0, 0.0, 1.0), (1.0, 1.0, 1.0));
        facet_from_vertexes((0.0, 0.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0));
        // Y-axis Bottom
        facet_from_vertexes((0.0, 0.0, 0.0), (1.0, 0.0, 0.0), (1.0, 0.0, 1.0));
        facet_from_vertexes((0.0, 0.0, 0.0), (1.0, 0.0, 1.0), (0.0, 0.0, 1.0));
        // Y-axis Top
        facet_from_vertexes((1.0, 1.0, 1.0), (1.0, 1.0, 0.0), (0.0, 1.0, 0.0));
        facet_from_vertexes((0.0, 1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0));
        // X-axis Back
        facet_from_vertexes((0.0, 1.0, 1.0), (0.0, 1.0, 0.0), (0.0, 0.0, 0.0));
        facet_from_vertexes((0.0, 0.0, 1.0), (0.0, 1.0, 1.0), (0.0, 0.0, 0.0));
        // X-axis Front
        facet_from_vertexes((1.0, 0.0, 0.0), (1.0, 1.0, 0.0), (1.0, 1.0, 1.0));
        facet_from_vertexes((1.0, 0.0, 0.0), (1.0, 1.0, 1.0), (1.0, 0.0, 1.0));
        println!("endsolid made_by_orthoptera");
    } else {
        panic!("COMPILE ERROR");
    }
}
