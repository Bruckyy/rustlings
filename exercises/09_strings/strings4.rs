// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

/// 1) hard coded string like that are &str
/// 2) to_string() is called which return a String so it is a String
/// 3) method "from()" called by String object so its a String
/// 4) to owned return us directly the object so wen can consume it, so its a String
/// 5) No type specified so it stay a string slice (&str)
/// 6) format macro return a String
/// 7) we're making a slice of "abc" here [0..1] from a String so its a string slice (&str)
/// 8) trim() is doing a string slice (&str)
/// 9) replace return a String
/// 10) to_lowercase return a String

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
