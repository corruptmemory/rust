// error-pattern: can only dereference structs with one anonymous field
struct cat {
    foo: ()
}

fn main() {
    let nyan = cat { foo: () };
    log (error, *nyan);
}
