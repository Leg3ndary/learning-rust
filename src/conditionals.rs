pub fn run_func () {
    println!("Lets see how this works");

    let birth_year: u16 = 2006;

    if birth_year <= 2000 {
        println!("Imagine being under 2000")
    } else {
        println!("I guess you're pretty cool then")
    }

    let x = if birth_year > 2000 { true } else { false };

    println!("{}", x);
}