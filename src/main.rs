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

    // RustのRangeは半開区間（half-open）であり、開始値は含むが終了値は含まない
    assert_eq!((3..5), std::ops::Range { start: 3, end: 5 });
    assert_eq!(3 + 4 + 5, (3..6).sum());
    let arr = [0, 1, 2, 3, 4];
    assert_eq!(arr[ ..  ], [0,1,2,3,4]);
    assert_eq!(arr[ .. 3], [0,1,2    ]);
    assert_eq!(arr[ ..=3], [0,1,2,3  ]);
    assert_eq!(arr[1..  ], [  1,2,3,4]);
    assert_eq!(arr[1.. 3], [  1,2    ]);  // Range
    assert_eq!(arr[1..=3], [  1,2,3  ]);

    let hi: u8 = 0xe0;
    let lo = !hi; // ビット反転は~でなく!
    assert_eq!(lo, 0x1f);

    let is_even = |x| x % 2 ==0;
    let is_even_i64 = |x: u64| -> bool { x % 2 == 0 }; // 返り値の型を明示する場合にはボディ部をブロックにしなければならない

    assert_eq!(is_even(14), true);
    assert_eq!(is_even_i64(14), true);
}
