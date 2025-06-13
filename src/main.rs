use clap::{command,Arg};
use chrono::{Utc};
use chrono_tz::America::New_York;
use std::fs;
use reqwest::Error;
use std::path::PathBuf;
use std::io;
use walkdir::WalkDir;



// #[tokio::main]
// async fn get(url: &str) ->Result<(), Error>
// 
#[tokio::main]
async fn get(url: &str,time: String) -> Result<(), Error> {
    let response = reqwest::get(url)
    .await?
    .text()
    .await?;
    println!("{}sent at :{}",response,time);
    write(time,response)
    .expect("ERROING WRITEING RESPONSE FILE");
    Ok(())
}

fn write(name: String, body: String) -> std::io::Result<()>{
    // let mut dir = String::new();
    let mut dir2 = PathBuf::from("get_responses");
    dir2.push("logs");
    let filename = name + ".txt";
    let content = body;
    fs::create_dir_all(&dir2)?;
    println!("directory exits!: {:?}", dir2);
    let file_path = dir2.join(filename);
    fs::write(&file_path,content)?;
    println!("wrote file to : {:?}", file_path);
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
// let url_input_closure = || -> &str{
//         let url = matches
//                 .get_one::<String>("url")
//                 .expect("ERROR URL")
//                 .trim();
//                 {
//                 println!("Get request to  {}",url);
//     }
//     url

// let file_string_closure = || -> String {
//         let file_names_all = i.path()
//         .display()
//         .to_string();
//          {
//             println!("get something");
//          }
        
//         file_names_all
// };


// fn list_files() -> String {
//     for i in fs::read_dir("./get_responses/logs").expect("cannot read directory"){
//         let i = i.expect("Invalid entry");
//         let file_names = i.path().display().to_string();
//         println!("{}", i.path().display());
//     }
//     let file_names2 = file_names.clone();
// }

fn list_files_goated() -> io::Result<Vec<String>> {
    fs::read_dir("./get_responses/logs")?
    .map(|res| res.map (|e| e.path().display().to_string()))
    .collect()
}

// this functino list_files_goated() is how i should 
// write all functions that need to return something

fn read_all_files_in_folder() {
    let dir_path = "./get_responses/logs";

    println!("Scanning for all files in: {}", dir_path);

    // WalkDir will recursively explore subdirectories.
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // all we need is to ensure its a file, not a directory.
        if path.is_file() {
            println!("--- Reading file: {:?} ---", path);

            match fs::read_to_string(path) {
                Ok(contents) => {
                    // only printing first 200 words to console
                    // let preview: String = contents.chars().take(230).collect();
                    let preview: String = contents
                    .lines()
                    .filter(|line|line.contains("url"))
                    .collect();

                    println!("Request sent to : {}\n", preview);

                }
                Err(e) => {
                    // if not uft-8 valid error 
                    eprintln!(
                        "Error reading file {:?} as string: {}. It might be a binary file.",
                        path, e
                    );
                }
            }
        }
    }
}


fn list_files() {
    let files = list_files_goated().unwrap();
    // println!("files : {:?}\n", files);
    for file in files {
        println!("{:?}",file)
    }
}

// search needs to take all the files
// then search all of them for "url" line






// make a function which reads over the folder
// get_responses and then i could do the minigrep
// and find "url" and display that line so
// in the terminal it would print out 
// (name[in this case it would be the time]) :: url :: "blahblah.org"

// // pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
//     contents
//         .lines()
//         .filter(|line|line.contains(query))
//         .collect()
// }




// fn list_responses() {
//     //takes a string of the files

// }


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

//so with the idea of sending the request into a file
// (like whenver i send a request and get a response
// store that info into a new file with the time
// as the name and then also store the request time
// in a seperate log so you can easily see all your
// times from when you sent a request)
// let request_log: Vec<String> = Vec::new();

// create function which loops through the file get/responses
// and appends a new to the file one every time a new file is added

    
    loop {
        println!("Start of loop!");

        match input {
            "time" => {
                let tsz = get_time();
                println!("{}",tsz);
                list_files();
                break;
            }
            "file" => {
                read_all_files_in_folder();
                break;
            }
            // "write" => {
            //     write()
            //     .expect("ERROING WRITEING TO FILE ");
            //     break;
            // }
            "get" => {
                let url = url_input_closure();
                get(url,get_time())
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