// error-pattern: can only dereference structs with one anonymous field
struct cat {
    x: ()
}

fn main() {
    let kitty : cat = cat { x: () };
    log (error, *kitty);
}
