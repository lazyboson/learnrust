fn main() {
    //primitive types
    let x = 32;
    let y = 2.5;

    let yy: i64 = 345;

    //find max size
    println!("max i32 {}", std::i32::MAX);

    //boolean
    let is_active = true;
    println!("{} {} {} is active ? {}", x, y, yy, is_active);

    let a1 = '1';
    println!("this is character {}", a1);

    // --- integer types (TRPL 3.2) ---
    // signed:   i8, i16, i32 (default), i64, i128, isize (pointer-sized)
    // unsigned: u8, u16, u32, u64, u128, usize
    let small: i8 = -128;            // range: -128 to 127
    let big: u64 = 18_000_000_000;   // underscores are ignored, just for readability
    println!("i8 = {}, u64 = {}", small, big);

    // integer literal forms
    let dec = 98_222;            // decimal
    let hex = 0xff;              // hexadecimal: 255
    let oct = 0o77;              // octal: 63
    let bin = 0b1111_0000;       // binary: 240
    let byte_lit = b'A';         // byte literal (u8 only): 65
    let typed = 57u32;           // type-suffixed literal
    println!("dec={}, hex={}, oct={}, bin={}, b'A'={}, typed_u32={}",
             dec, hex, oct, bin, byte_lit, typed);

    // integer overflow:
    //   debug build  -> panics
    //   release build -> wraps (two's complement)
    // for explicit handling use checked_*, wrapping_*, overflowing_*, saturating_*.
    let a: u8 = 250;
    let b: u8 = a.wrapping_add(10);  // 250 + 10 wraps mod 256 = 4
    println!("250u8.wrapping_add(10) = {}", b);

    // --- floating point ---
    let f_default = 3.0;             // f64 by default
    let f_single: f32 = 3.0;
    println!("f64 = {}, f32 = {}", f_default, f_single);

    // numeric operations: + - * / %
    let int_div = 7 / 3;             // integer division truncates toward zero -> 2
    let rem = 43 % 5;
    println!("7/3 = {}, 43 % 5 = {}", int_div, rem);

    // --- char is a 4-byte Unicode scalar value, NOT just ASCII ---
    let ch_ascii = 'z';
    let ch_emoji = '😻';
    let ch_kanji = '日';
    println!("ascii={}, emoji={}, kanji={}", ch_ascii, ch_emoji, ch_kanji);
    println!("size_of::<char> = {} bytes", std::mem::size_of::<char>());

    // --- the unit type () ---
    // a tuple with no values; expressions that "return nothing" return ().
    let unit: () = ();
    println!("unit = {:?}", unit);
}