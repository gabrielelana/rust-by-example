fn main() {
    // Type annotated variable
    let a_float: f64 = 1.0;
    println!("A float {}", a_float);

    // This variable is an `int`
    let mut an_integer = 5i;
    println!("An integer {}", an_integer);

    // Error! The type of a variable can't be changed
    // an_integer = true;

    an_integer = 6i;
    println!("An integer changed to {}", an_integer);
}
