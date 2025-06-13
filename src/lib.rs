use chrono::{Utc};
use chrono_tz::America::New_York;
use std::fs;
// use reqwest::Error;
use std::path::PathBuf;
use std::io;
use walkdir::WalkDir;
use::std::fs::OpenOptions;
use::std::io::Write;




// i could put this onto a struct that has all the request/ http funcs
// also in main,loop
// #[tokio::main]
// pub async fn get(url: &str,time: String) -> Result<(), Error> {
//     let response = reqwest::get(url)
//     .await?
//     .text()
//     .await?;
//     println!("{}sent at :{}",response,time);
//     write(time,response)
//     .expect("ERROING WRITEING RESPONSE FILE");
//     Ok(())
// }

// write used in http / get func

pub fn write(name: String, body: String, url: &str) -> std::io::Result<()>{
    let mut dir2 = PathBuf::from("/home/koushikk/Documents/Rust/rustcli/bomba/src/get_responses");
    dir2.push("logs");
    let filename = name + ".txt";
    fs::create_dir_all(&dir2)?;
    println!("directory exits!: {:?}", dir2);
    let file_path = dir2.join(filename);
    
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(&file_path)?;

    writeln!(file," Path : {}",file_path.display())?;

    writeln!(file," url : {}\n",url)?;

    writeln!(file,"{}",body)?;

    println!("wrote file to : {:?}", file.metadata()?.len());
    Ok(())
}


pub fn get_time() -> String{
    let utc_now = Utc::now();

    let ny_time = utc_now.with_timezone(&New_York);

    let hour = ny_time.format("%I:%M  %p").to_string();
    let hour = hour.trim_start_matches('0');
    let hour = hour.to_string();

    return hour
}


// ya ya ya
pub fn list_files_goated() -> io::Result<Vec<String>> {
    fs::read_dir("./get_responses/logs")?
    .map(|res| res.map (|e| e.path().display().to_string()))
    .collect()
}




pub fn read_all_files_in_folder() {
    let dir_path = "/home/koushikk/Documents/Rust/rustcli/bomba/src/get_responses/logs";

    println!("Scanning for all files in: {}", dir_path);

    // WalkDir will recursively explore subdirectories.
    // ya yya ya
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
                    
                // if there no line.contains url
                //false write request link to file 
                    
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


// used in main loop can easily get rid of this func tho 
// pub fn list_files() {
//     let files = list_files_goated().unwrap();
//     // println!("files : {:?}\n", files);
//     for file in files {
//         println!("{:?}",file)
//     }
// }
