pub fn start_loop () {
    let mut counter: u16 = 0;

    while counter < 10 {
        counter += 1;
        
        if counter % 15 == 0 {
            println!("fizzbuzz");
        } else if counter % 3 == 0 {
            println!("fizz");
        } else if counter % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", counter);
        }
    }

    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}
