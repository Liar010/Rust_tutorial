enum WebEvent {
    PageLoad,
    PageUnload,
    // like tuple structs,
    // �^�v�����ł��悢
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    // C����X�^�C���̍\���̕��ł��悢
    Click { x: i64, y: i64 },
}
// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
// �����Ƃ���`WebEvent`�񋓌^���Ƃ�A�����Ԃ��Ȃ��֐�
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
    /* ���l */
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    {/* �w�肵�Ȃ��Ă��ł��� */
        let logical = true;
        let a_float = 1.0;
        let an_integer = 5;
    }

    /* ������ */
    let s1: String = "Hello, World".to_string();
    //let s1: String = String::from("Hello, World");�ł���
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    /* �^�v�� */
    let t = (1, "2", 0.5);
    println!("taupple first value = {}", t.0);
    println!("taupple second value = {}", t.1);
    println!("taupple third value = {}", t.2);

    /* �z�� */
    let a:[i32;4] = [0, 1, 2, 3];
    //let b:[i32;4] = [0, 1, 2];//�R���p�C���G���[
    println!("a[0]:{}", a[0]);
    println!("&a[0]:{}", &a[0]);
    println!("a[0..4]:{:?}", &a[0..4]);//�X���C�X�Ƃ��Ĉ����Ɣ͈͎w�肪�ł��ĕ֗��炵��

    /* �\����(���[�U�[��`) */
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

    /* enum(Rust By Example�����p) */
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`�͕�����X���C�X���珊�L���̂���`String`���쐬����
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    
    /* �L���X�g�ϊ� */
    let decimal = 65.4321;
    let an_integer:u8 = decimal; //�ÖٓI�ȃL���X�e�B���O�͂ł��Ȃ�
    let an_integer = decimal as u8;//�����I�ɂȂ�ł���
    let character = decimal as char;//float��char�ɒ��ڕϊ��͂ł��Ȃ�
    let character = an_integer as char;//int�Ȃ�char�ɒ��ڕϊ��ł���
    println!("Casting: {} -> {} -> {}", decimal, an_integer, character);
    // 1000 �͂��ł�u16�Ɏ��܂��Ă��邽�ߕω����Ȃ��B
    println!("1000 as a u16 is: {}", 1000 as u16);
    // ���ʉ��ł͍ŉ��ʃr�b�g����8bit���g�p����A�c��̏�ʃr�b�g�����k�����`�ɂȂ�B
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    // 128��u8�ɃL���X�g�����128�ƂȂ�B128��8�r�b�g�ɂ�����␔�� -128
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
