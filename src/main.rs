use clap::{command,Arg};
use lib::{handle_match};




fn main() {
    let matches = command!()
    .arg(   
        Arg::new("input")
        .help("Your user input")
        .required(true),
    )
    .arg(
        Arg::new("url")
        .help("your URL")
        .requires("input")
        .short('u')
        .long("url")
        .value_name("URL")
    )
    .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("Input is required")
        .trim();

        // use closure so that it can see the matches variable
        // if it was a function i would pass it in? i think
        // i tried that and it didnt work 
    let url: Option<&str> = matches.get_one::<String>("url").map(|s|s.trim());



     // maybe turn the url input into a function
    // // if i make it a closure i can just make it so handle match 
    // accepts a closure instead of a url variable
    // or if i can just     
    handle_match(input, url);
    

}
     
