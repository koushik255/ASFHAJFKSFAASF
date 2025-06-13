use clap::{command,Arg};
use chrono::{Utc};
use chrono_tz::America::New_York;
use std::fs;
use reqwest::Error;


// #[tokio::main]
// async fn get(url: &str) ->Result<(), Error>
// 
#[tokio::main]
async fn get(url: &str) -> Result<(), Error> {
    let response = reqwest::get(url)
    .await?
    .text()
    .await?;
    println!("{}",response);
    Ok(())
}

// i can make it so the tokio response stores the info in 
// a file because it give acces to file system
// i can also include the time of the request aswell

fn get_time() -> String{
    let utc_now = Utc::now();

    let ny_time = utc_now.with_timezone(&New_York);

    let hour = ny_time.format("%I:%M  %p").to_string();
    let hour = hour.trim_start_matches('0');
    let hour = hour.to_string();
    // println!("Current time in EST is : {}",hour);

    return hour
}


fn list_files() {
    for i in fs::read_dir(".").expect("cannot read directory"){
        let i = i.expect("Invalid entry");
        println!("{}", i.path().display());
    }
}


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
    let url_input_closure = || -> &str{
        let url = matches
                .get_one::<String>("url")
                .expect("ERROR URL")
                .trim();
                {
                println!("Get request to  {}",url);
    }
    url
};
    
    loop {
        println!("Start of loop!");

        match input {
            "time" => {
                let tsz = get_time();
                println!("{}",tsz);
                break;
            }
            "file" => {
                list_files();
                break;
            }
            "get" => {
                let url = url_input_closure();
                get(url)
                .expect("Error sending get request!");
             break;
            }
            _ => {
                println!("UNlucky");
                break;
            }
        }
    }

}