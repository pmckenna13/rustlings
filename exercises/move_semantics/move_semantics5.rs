// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// DONE

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // This is allowed, ensures x is only borrowed once at a time
    let z = &mut x;
    //*y += 100; Cannot do this here, x is borrowed twice
    *z += 1000;
    assert_eq!(x, 1200);
}
