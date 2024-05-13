const SOURCE_CODE: &str = "cube()";

fn main() {
    if SOURCE_CODE == "cube()" {
        // Z-axis Back
        println!("solid made_by_orthoptera");
        println!("facet normal 0.0 0.0 -1.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 0.0");
        println!("    vertex 1.0 1.0 0.0");
        println!("    vertex 1.0 0.0 0.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal 0.0 0.0 -1.0");
        println!("  outer loop");
        println!("    vertex 0.0 1.0 0.0");
        println!("    vertex 1.0 1.0 0.0");
        println!("    vertex 0.0 0.0 0.0");
        println!("  endloop");
        println!("endfacet");
        // Z-axis Front
        println!("facet normal 0.0 0.0 1.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 1.0");
        println!("    vertex 1.0 0.0 1.0");
        println!("    vertex 1.0 1.0 1.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal 0.0 0.0 1.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 1.0");
        println!("    vertex 1.0 1.0 1.0");
        println!("    vertex 0.0 1.0 1.0");
        println!("  endloop");
        println!("endfacet");
        // Y-axis Bottom
        println!("facet normal 0.0 -1.0 0.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 0.0");
        println!("    vertex 1.0 0.0 0.0");
        println!("    vertex 1.0 0.0 1.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal 0.0 -1.0 0.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 0.0");
        println!("    vertex 1.0 0.0 1.0");
        println!("    vertex 0.0 0.0 1.0");
        println!("  endloop");
        println!("endfacet");
        // Y-axis Top
        println!("facet normal 0.0 1.0 0.0");
        println!("  outer loop");
        println!("    vertex 1.0 1.0 1.0");
        println!("    vertex 1.0 1.0 0.0");
        println!("    vertex 0.0 1.0 0.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal 0.0 1.0 0.0");
        println!("  outer loop");
        println!("    vertex 0.0 1.0 1.0");
        println!("    vertex 1.0 1.0 1.0");
        println!("    vertex 0.0 1.0 0.0");
        println!("  endloop");
        println!("endfacet");
        // X-axis Back
        println!("facet normal -1.0 0.0 0.0");
        println!("  outer loop");
        println!("    vertex 0.0 1.0 1.0");
        println!("    vertex 0.0 1.0 0.0");
        println!("    vertex 0.0 0.0 0.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal -1.0 0.0 0.0");
        println!("  outer loop");
        println!("    vertex 0.0 0.0 1.0");
        println!("    vertex 0.0 1.0 1.0");
        println!("    vertex 0.0 0.0 0.0");
        println!("  endloop");
        println!("endfacet");
        // X-axis Front
        println!("facet normal 1.0 0.0 0.0");
        println!("  outer loop");
        println!("    vertex 1.0 0.0 0.0");
        println!("    vertex 1.0 1.0 0.0");
        println!("    vertex 1.0 1.0 1.0");
        println!("  endloop");
        println!("endfacet");
        println!("facet normal 1.0 0.0 0.0");
        println!("  outer loop");
        println!("    vertex 1.0 0.0 0.0");
        println!("    vertex 1.0 1.0 1.0");
        println!("    vertex 1.0 0.0 1.0");
        println!("  endloop");
        println!("endfacet");
        println!("endsolid made_by_orthoptera");
    } else {
        panic!("COMPILE ERROR");
    }
}
