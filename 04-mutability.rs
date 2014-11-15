fn main() {
    let immutable_variable = 1i;
    let mut mutable_variable = 1i;

    println!("Before mutation: {}", mutable_variable);

    // Ok
    mutable_variable += 1;

    println!("After mutation: {}", mutable_variable);

    println!("This is immutable: {}", immutable_variable);

    // Error!
    // immutable_variable += 1;
}
