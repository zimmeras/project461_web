use std::env; //rust stdlib function to get command line args
use std::fs; //rust file library
use std::fs::File; //rust file library
use std::process::Command; //library to run processes in rust
use std::io::BufWriter;
use tokei::{Config, Languages, LanguageType};
//use pyo3::prelude::*; //module to run python code in rust
use std::path::Path;
use std::io::Write;

fn main(){
    
    //save the command line argument
    let cli_input: Vec<String> = env::args().collect(); 

    //create a variable for the file path and save the first command line argument into it
    let filepath = &cli_input[1]; 

    println!("{}", filepath);

    //take the contents of the file and save into a single string
    let data = fs::read_to_string(filepath).expect("Unable to read file");

    //now, chop this string into a vector at every newline since the URLS are newline delimited
    let _urls: Vec<&str> = data.split('\n').collect();

    //get number of URLS
    let num_urls = _urls.len();
    println!("number of urls passed in: {}", num_urls);

    //loop through urls and print their corresponding clone folder numbers
    let mut url_index = 1;
    for url in _urls {
        println!("folder number / url     :     {} / {}", url_index, url);
        url_index += 1;
    }
    
    //check if the cloned repos folder is already there from a previous run, and delete it if so
    let is_cloned_repos = Path::new("local_cloning/cloned_repos/").exists();
    if is_cloned_repos == true{
        fs::remove_dir_all("local_cloning/cloned_repos/").expect("error deleting cloned repos directory");
    }
    
    //run clone function to locally clone repos (pass in the input file)
    clone_repos((&filepath).to_string());

    //open up the directory with all the cloned repos
    let cloned_folders = fs::read_dir("local_cloning/cloned_repos/").unwrap();

    //initialize a counter for the current folder number
    let mut folder_num = 1;

    //open output file to write rampup scores to
    let mut out_file = BufWriter::new(File::create("output/rampup_out.txt").expect("Error opening output file for rampup"));

    //loop through all folders in "cloned_repos"
    for folder in cloned_folders {

        //get the path of the current folder
        let _folder_path = (folder.unwrap().path().display()).to_string(); 

        //initialize parameters for when we use tokei to parse the folders and count code/comments
        let paths = &[_folder_path]; //folder to search
        let excluded = &[]; //folders excluded from the search
        let config = Config::default(); //configure the parser to default

        //configure the tokei language to be javascript 
        let mut languages = Languages::new();
        languages.get_statistics(paths, excluded, &config);
        let js = &languages[&LanguageType::JavaScript];

        //get the number of lines of code and comments
        let code_lines = js.code;
        let comment_lines = js.comments;
        println!("Lines of code in folder {}: {}", folder_num, code_lines);
        println!("Lines of comments in folder {}: {}", folder_num, comment_lines);

        //calculate rampup
        let code_u32 = u32::try_from(code_lines).unwrap();
        let comment_u32 = u32::try_from(comment_lines).unwrap();
        let ramp_up = calculate_ramp_up(code_u32, comment_u32);
        println!("RampUp score for repo {}: {:.2}\n", folder_num, ramp_up);

        

        write!(out_file, "{0}\n", ramp_up).expect("Error writing rampup to output");

        //increment the folder counter
        folder_num+=1;
    }
}

//this function takes the number of code lines, and comment lines and calculates the rampup score
fn calculate_ramp_up(lines_of_code: u32, lines_of_comments: u32) -> f32{

    //calculate RampUp score using our formula
    let float_code= lines_of_code as f32;
    let float_comments = lines_of_comments as f32;
    let calculated_score: f32 = 2.0 * (float_comments / float_code);

    //return 1 if the calculated score was greater than 1 and return the calculated score if it was less than 1
    if calculated_score > 1.0 {
        return 1.0;
    } else {
        return calculated_score;
    }

}

//this function simply runs the file "clone_repo.py" which is located instide of the "461_project/local_cloning" directory
fn clone_repos(filepath: String){

    //run clone_repo.py
    let _run_clone_script = Command::new("python3").arg("local_cloning/clone_repo.py").arg(&filepath).status();
}

