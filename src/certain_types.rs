pub fn run_thingy () {
    let mut bear = String::from("How are you doing today");

    println!("{}", bear);

    bear.push('?');

    println!("{}", bear);

    bear.push_str("How are you doing?");

    println!("{}", bear);

    println!("{:?}", (bear.capacity(), bear.len()));
}