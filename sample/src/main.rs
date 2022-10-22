enum WebEvent {
    PageLoad,
    PageUnload,
    // like tuple structs,
    // タプル風でもよい
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    // C言語スタイルの構造体風でもよい
    Click { x: i64, y: i64 },
}
// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
// 引数として`WebEvent`列挙型をとり、何も返さない関数
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    /* 数値 */
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    {/* 指定しなくてもできる */
        let logical = true;
        let a_float = 1.0;
        let an_integer = 5;
    }

    /* 文字列 */
    let s1: String = "Hello, World".to_string();
    //let s1: String = String::from("Hello, World");でも可
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    /* タプル */
    let t = (1, "2", 0.5);
    println!("taupple first value = {}", t.0);
    println!("taupple second value = {}", t.1);
    println!("taupple third value = {}", t.2);

    /* 配列 */
    let a:[i32;4] = [0, 1, 2, 3];
    //let b:[i32;4] = [0, 1, 2];//コンパイルエラー
    println!("a[0]:{}", a[0]);
    println!("&a[0]:{}", &a[0]);
    println!("a[0..4]:{:?}", &a[0..4]);//スライスとして扱うと範囲指定ができて便利らしい

    /* 構造体(ユーザー定義) */
    struct Person {
        name: String,
        age: u32,
        height: u32,
        weight: u32,
    }
    let taro = Person{
        name: "TaroYamada".to_string(),
        age: 20,
        height: 170,
        weight: 65,
    };

    /* enum(Rust By Exampleより引用) */
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`は文字列スライスから所有権のある`String`を作成する
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    
    /* キャスト変換 */
    let decimal = 65.4321;
    let an_integer:u8 = decimal; //暗黙的なキャスティングはできない
    let an_integer = decimal as u8;//明示的にならできる
    let character = decimal as char;//floatはcharに直接変換はできない
    let character = an_integer as char;//intならcharに直接変換できる
    println!("Casting: {} -> {} -> {}", decimal, an_integer, character);
    // 1000 はすでにu16に収まっているため変化しない。
    println!("1000 as a u16 is: {}", 1000 as u16);
    // 水面下では最下位ビットから8bitが使用され、残りの上位ビットが圧縮される形になる。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    // 128をu8にキャストすると128となる。128の8ビットにおける補数は -128
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!("1000 mod 256 is : {}", 1000 % 256);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a i8 is : {}", 232 as i8);

    // 300.0 is 255
    println!("300.0 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);


}
