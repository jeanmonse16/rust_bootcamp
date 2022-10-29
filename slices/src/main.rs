fn main() {
    let tweet: String = String::from(
        "This is my tweet and it's very very long"
    );
    let trimmed_tweet: &str = trim_tweet(&tweet);
    let tweet2: &str = "This is my tweet and it's very very long";
    let trimmed_tweet2: &str = trim_tweet(tweet2);
    println!("{trimmed_tweet}");
    println!("{trimmed_tweet2}");
    let a: [i32;6] = [1,2,3,4,5,6];
    let a_slice: &[i32] = &a[..3];
    println!("{:?}", a_slice);

    let s1: &str = "Helloooooo ";
    let s2 = String::from("Hello I am a string");
    let s3: String = "Hello my friend".to_string();
    let s4 = "Owning a string".to_owned();
    let s5 = &s4[..];
    let mut s6 = String::from("foo");
    s6.push_str("baz");
    println!("{}", s6);
    s6.replace_range(.., "siuuu");
    println!("{}", s6);

    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let s10 = format!("{s7}-{s8}-{s9}");
    println!("{}", s10);
    let s11 = concat!(s8, s9);
}

fn trim_tweet(tweet: &str) -> &str {
    &tweet[..20]
}
