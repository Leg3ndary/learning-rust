// Looking at some fun tuples

pub fn run () {
    let groupy: (&str, f32) = ("Hello", 3.1415926535);

    println!("{:?}", (groupy.0, groupy.1));

    let testing: [&str; 5] = ["hi", "voila", "thirteen", "something", "he"];

    println!("{:?}", testing);
}