use std::io::Write;
fn main() {
    let mut age: i32 = 16;
    writeln!(&mut std::io::stdout().lock(), "{age} joy").unwrap();
    age = age / 2;
    writeln!(&mut std::io::stdout().lock(), "{age}").unwrap();
}
