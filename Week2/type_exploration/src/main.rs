fn main() {
    println!("Integer Types");
    let a: i8 = -128;
    let b: i16 = 32767;
    let c: i32 = -2_000_000_000;
    let d: isize = -42;

    let e: u8 = 255;
    let f: u16 = 65_535;
    let g: u32 = 4_000_000_000;
    let h: usize = 42;

    println!("Signed: i8={}, i16={}, i32={}, isize={}", a, b, c, d);
    println!("Unsigned: u8={}, u16={}, u32={}, usize={}", e, f, g, h);

    println!("\nFloating Point Types");
    let f32_num: f32 = 3.141592;
    let f64_num: f64 = 2.718281828459045;
    println!("f32 = {}", f32_num);
    println!("f64 = {}", f64_num);

    println!("\nBoolean Operations");
    let t = true;
    let f = false;
    println!("true AND false = {}", t && f);
    println!("true OR false = {}", t || f);
    println!("NOT true = {}", !t);

    println!("\nCharacters");
    let lett = 'T';
    let dig = "13";
    let sym = '%';
    let emoji = '😎';
    println!("Characters: {}, {}, {}, {}", lett, dig, sym, emoji);
}

