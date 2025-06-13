use clap::{command,Arg};
use reqwest::Error;
use lib::{get_time,read_all_files_in_folder,write};



#[tokio::main]
async fn get(url: &str,time: String) -> Result<(), Error> {
    let response = reqwest::get(url)
    .await?
    .text()
    .await?;
    println!("{}sent at :{}",response,time);
    write(time,response,url)
    .expect("ERROING WRITEING RESPONSE FILE");
    Ok(())
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
                // list_files();
                break;
            }
            "file" => {
                read_all_files_in_folder();
                break;
            }
            "get" => {
                // have to use -u for it to work!
                let url = url_input_closure();
                println!("{}",url);
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