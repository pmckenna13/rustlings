// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// DONE

fn main() {
    let data = "Rust is great!".to_string();

    let x = get_char(&data);
    println!("{}", x);

    string_uppercase(data);
    // Cannot call line below since previous function takes ownershit of data
    // This occurs because there is no Copy for String type so it must move
    // println!("{}", data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
