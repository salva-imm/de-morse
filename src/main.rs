use std::collections::HashMap;
use std::io;

fn main() {
    let morse_dictionary = HashMap::from([
        ("a", ".-"),
        ("b", "-..."),
        ("c", "-.-."),
        ("d", "-.."),
        ("e", "."),
        ("f", "..-."),
        ("g", "--."),
        ("h", "...."),
        ("i", ".."),
        ("j", ".---"),
        ("k", "-.-"),
        ("l", ".-.."),
        ("m", "--"),
        ("n", "-."),
        ("o", "---"),
        ("p", ".--."),
        ("q", "--.-"),
        ("r", ".-."),
        ("s", "..."),
        ("t", "-"),
        ("u", "..-"),
        ("v", "...-"),
        ("w", ".--"),
        ("x", "-..-"),
        ("y", "-.--"),
        ("z", "--.."),
        ("0", "-----"),
        ("1", ".----"),
        ("2", "..---"),
        ("3", "...--"),
        ("4", "....-"),
        ("5", "....."),
        ("6", "-...."),
        ("7", "--..."),
        ("8", "---.."),
        ("9", "----."),
        (".", ".-.-.-"),
        (",", "--..--"),
        ("?", "..--.."),
        ("!", "-.-.--"),
    ]);

    let morse_dictionary_inverted: HashMap<&str, &str>= morse_dictionary.iter()
    .map(|(k, v)| (*v, *k)).collect();

    println!("Morse coder-decoder!");
    let mut choice = String::new();
    while !["1", "2"].contains(&&choice[..]) {
        println!("1)decode\n2)encode");
        io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
        choice = choice.trim().to_string();
        if !["1", "2"].contains(&&choice[..]){
            choice = "".to_string();
        }
    }

    if choice == String::from("1"){
        println!("Please input your morse code. then press double ENTER to interpret");
    }else{
        println!("Please input your word or sentence code. then press double ENTER to interpret");
    }

    let mut code = String::new();
    while !code.ends_with("\n\n"){
    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line");
    }
    let binding = code.replace("\n\n", "");
    let trimmed_binding = binding.trim();
    let codex: Vec<_> = trimmed_binding.split("\n").collect();
    let mut interpret_ref = &morse_dictionary_inverted;
    if choice == String::from("2"){
        interpret_ref = &morse_dictionary;
    }
    println!("Code: {:#?}", codex);
    for &item in codex.iter(){
        println!("{}", interpret_ref.get(item).unwrap());
    }

}
