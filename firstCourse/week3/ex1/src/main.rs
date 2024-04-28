struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
    age: u8,
}

impl User {
    fn new(username: String, email: String, uri: String, age: u8) -> Self {
        Self {
            username,
            email,
            uri,
            age, 
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
        18
    );
    println!("Hello, {}! Your age is: {}", new_user.username, new_user.age);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
}

// fn main() {
//     let sentence = "the quick brown fox jumps over the lazy dog".to_string();
//     // Use slicing to get the first three characters of the sentence
//     //println!("{}", &sentence[0..=4]);

//     // concatenate using format!
//     let description = format!("Title: Quick story\n{}", sentence);
//     //println!("{}", description);

//     // iterate over the characters in the sentence
//     // for c in sentence.chars() {
//     //     match c {
//     //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
//     //         _ => continue,
//     //     }
//     // }

//     // Split and collect into a vector
//     //let words: Vec<&str> = sentence.split_whitespace().collect();
//     let words = sentence.split(' ').collect::<Vec<&str>>();
//     //println!("{:?}", words);

//     let reversed = sentence.chars().rev().collect::<String>();
//     println!("{}", reversed);
// }

