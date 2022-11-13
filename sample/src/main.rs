fn main() {

    /* Option type : Something or Nothing */
    let mut word: String = "Hello, World".to_string();
    for _i in 0..13{/*Recommend use len() method: word.len() */
        match word.pop() {/* pop(): Removes the last character from the string buffer and returns it. */
            Some(c) => println!("{}", c),
            None => println!("String is empty"),
        }
    }

    /* Result type : Success(Ok) or failure(Err)*/
    let four: &str = "4";
    match four.parse::<u8>() {
        Ok(n) => println!("parse success: {}", n),
        Err(err) => println!("{:?}", err),
    }
    /* or 
        let num = four.parse::<u8>().unwrap()
        (if "four" is not number string, unwrap() return "panic" and finish program with error message)
     */
}