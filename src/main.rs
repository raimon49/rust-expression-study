// fn f() -> ! と宣言すると終了しない式になるため、returnがあるとコンパイルエラーになる
fn f() {
    // return (); の省略形
    return;
}
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

    // ..演算子は範囲（range）を生成する
    // std::ops::Range{ start: 0, end: 20 } と同じ意味
    for i in 0..20 {
        println!("{}", i)
    }

    let mut strings = Vec::new();
    strings.push("100");
    strings.push("200");
    strings.push("300");
    // for s in strings {
    // 参照に対してループ式を適用しないとオーナーシップが移動してしまう
    for s in &strings {
        println!("{}", s)
    }
    println!("end of loop with {} elements", strings.len());

    let r = f(); // return; では実際には空のタプルが返されている
    assert_eq!(r, ());

    // スタティックメソッド呼び出しのジェネリクスでは ::<...> ターボフィッシュを使う
    // Vec<i32>::new や Vec<i32>::with_capacity は比較演算子と見做されコンパイルエラー
    let numbers = Vec::<i32>::with_capacity(1000);
    assert_eq!(numbers.len(), 1000);
}
