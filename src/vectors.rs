pub fn vectors_smiley () {
    // My vectors stuff yay
    let mut new_vector: Vec<&str> = vec!["Hello", "seentenn"];

    new_vector.push("Hello how are you");
    new_vector.push("Another value");

    for x in new_vector.iter() {
        println!("value = {}", x);
    }
}