fn main() {
    let code = 100;
    match code {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", code)
    }

    let value = Some(1);
    // if let式はパターンがひとつしかないmatch式
    if let Some(v) = value {
        println!("Some value is {}", v)
    }
    // match式でパターンマッチ
    match value {
        Some(v) => println!("Some value is {}", v),
        None => println!("Some value is None")
    }
}
