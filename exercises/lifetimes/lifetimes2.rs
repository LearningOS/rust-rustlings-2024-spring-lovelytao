// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// fn longest<'a>(x: &str, y: &str) -> &'a str {
fn longest<'a>(x: &'a str, y: &'a str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result 必须要活到 println!处，因为 result 的生命周期是 'a，因此 'a 必须持续到 println!。
        // 在 longest 函数中，string2 的生命周期也是 'a，
        // 由此说明 string2 也必须活到 println! 处，
        // 可是 string2 在代码中实际上只能活到内部语句块的花括号处 }，
        // 小于它应该具备的生命周期 'a，因此编译出错。
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
