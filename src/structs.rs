// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct TupColor(u8, u8, u8);

struct Person {
    name: String,
    surname: String,
    height_in_cm: &i32
}

pub fn show_structs() {
    let mut c = Color {
        red: 255,
        green: 255,
        blue: 0
    };

    println!("Color rgb values {} {} {}", c.red, c.green, c.blue);

    c.blue = 255;

    let mut t = TupColor(255, 0, 0);
    println!("color {} {} {}", t.0, t.1, t.2);
}