pub fn run () {
    println!("Eric is a mega noob");

    println!("max i128: {}", std::i128::MAX);

    let float_max = std::f64::MAX;

    println!("{float}", float=float_max);

    let eric = "cool";
    println!("not {}", eric);

    println!("debug: {:?}", (float_max, eric));
}