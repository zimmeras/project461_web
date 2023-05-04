// Calculate bus factor score based on the bus factor and number of contributors
use octocrab::Octocrab;
use serde::Deserialize;
use std::fs;
use std::fs::File; //rust file library
use std::io::BufWriter;
use std::io::Write;
use std::sync::Arc; //rust write library

pub fn bus_factor_score(filepath: &str) {
    simple_log::info!("Calculating Bus Factor Score.)");

    let token = std::env::var("GITHUB_TOKEN");
    let octocrab = match token {
        Ok(t) => {
            simple_log::debug!("BF Score: Used Github token.)");
            Arc::new(Octocrab::builder().personal_token(t).build().unwrap())
        }
        Err(_e) => {
            simple_log::debug!("BF Score: Did not use Github token.");
            octocrab::instance()
        }
    };

    // Get the urls from the input file
    let urls = get_urls(filepath);

    // Create the output file
    let mut out_file = BufWriter::new(
        File::create("output/bus_factor_out.txt").expect("Error creating output file!"),
    );

    // Iterate through the urls and calculate the bus factor score
    for url in &urls {
        let git_url;

        // If the url is from npm, get the github url
        //log url
        simple_log::info!("url = {}", url);
        if &url[0..22] == "https://www.npmjs.com/" {
            git_url = get_github_url_for_npm(&url).unwrap();
        } else {
            git_url = url.to_string();
        }

        // Get the keywords from the url in the form (owner, repo)
        let keywords = get_keywords(&git_url);

        // Calculate the bus factor score
        let score = find_bf_score(&octocrab, keywords);

        // Write the score to the output file
        write!(out_file, "{0}\n", score).expect("Error writing bus factor to output");
    }
}

// Function to get the urls from the input file
pub fn get_urls(filepath: &str) -> Vec<String> {
    let data = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(..) => {
            println!("Error reading the input file!\n");
            std::process::exit(1);
        }
    };

    let urls: Vec<&str> = data.split('\n').collect();
    let mut url_vec = Vec::new();
    for url in urls {
        url_vec.push(url.to_string());
    }
    url_vec
}

// Function to calculate the bus factor score
#[tokio::main]
async fn find_bf_score(octocrab: &Octocrab, (owner, repo): (&str, &str)) -> f32 {
    // Get the repo information using octocrab
    let repo = octocrab.repos(owner, repo).get().await.unwrap();

    let url = repo.contributors_url.unwrap();
    let path = url.path();

    // Get the contributor information using http request (through octocrab)
    let user_info: Vec<Contributor> = octocrab.get(path, None::<&str>).await.unwrap();
    simple_log::debug!("contents = {:?}", user_info);

    // Get the number of contributors
    let num_contributors = user_info.len() as f32;

    // Calculate the bus factor
    let mut total_contributions = 0;
    for structure in &user_info {
        total_contributions += structure.contributions;
    }
    let mut contributions = 0;
    let mut bus_factor = 0;
    while contributions < (total_contributions / 2) {
        contributions += user_info[bus_factor].contributions;
        bus_factor += 1;
    }
    simple_log::info!("bus factor = {}", bus_factor);
    // Normalize the bus factor score
    normalize_score(num_contributors, (bus_factor + 1) as f32)
}

// Function to normalize the bus factor score
fn normalize_score(num_contributors: f32, bus_factor: f32) -> f32 {
    let bus_factor_norm = f32::exp(-0.25 * bus_factor);
    let length_norm = f32::exp(-0.1 * num_contributors);
    bus_factor_norm * 0.8 + length_norm * 0.3
}

// Function to get the keywords from the url
pub fn get_keywords(_url: &str) -> (&str, &str) {
    let part_str = &_url[19..];
    let divisionidx = part_str.find("/").expect("Error getting keywords");
    let owner = &part_str[..divisionidx];
    let repo = &part_str[divisionidx + 1..];

    (owner, repo)
}

// Function to get the github url from the npm url
pub fn get_github_url_for_npm(npm_url: &str) -> Result<String, ureq::Error> {
    let url = format!("https://registry.npmjs.org/{}", &npm_url[30..]);
    let json: serde_json::Value = ureq::get(&url).call()?.into_json()?;
    let repo_info = &json["repository"];

    if repo_info["type"] == "git" {
        let mut github_url = repo_info["url"].as_str().unwrap()[4..].to_string();
        if &github_url[..10] == "ssh://git@" {
            github_url = github_url[10..].to_string();
            github_url = format!("https://{github_url}");
        } else if &github_url[..2] == "//" {
            github_url = format!("https:{github_url}");
        }
        for _i in 1..5 {
            github_url.pop();
        }
        return Ok(github_url);
    } else {
        return Ok("".to_string());
    }
}

// Struct to hold the contributor information
#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Contributor {
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    contributor_type: String,
    site_admin: bool,
    contributions: i32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_score() {
        simple_log::info!("Calculate normalized score test 1");
        let result = normalize_score(1.0, 1.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.89);

        let result = normalize_score(1.0, 2.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.76);

        let result = normalize_score(0.0, 0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 1.1);

        let result = normalize_score(0.0, 1.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.92);

        let result = normalize_score(0.0, 2.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.79);

        let result = normalize_score(1.0, 0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 1.07);

        let result = normalize_score(2.0, 0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 1.05);

        let result = normalize_score(3.0, 0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 1.02);

        simple_log::info!("Calculate normalized score test 1 passed");
    }

    #[test]
    fn test_normalize_score2() {
        simple_log::info!("Calculate normalized score test 2");
        let result = normalize_score(5.0, 5.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.41);

        let result = normalize_score(0.0, 1000000.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.3);

        let result = normalize_score(1000000.0, 0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.8);

        let result = normalize_score(1000000.0, 1000000.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.0);

        let result = normalize_score(100000000.0,10000.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.0);

        let result = normalize_score(2.0,1.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.87);

        let result = normalize_score(1.234,5.678);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 0.46);

        let result = normalize_score(1.234,0.0);
        let round_res = (result * 100.0).round() / 100.0;
        assert_eq!(round_res, 1.07);

        simple_log::info!("Calculate normalized score test 2 passed");
    }

    #[test]
    fn test_get_urls_1() {
        simple_log::info!("test URL 1");
        let path = "./test/npm_urls_1.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 5);
        assert_eq!(url_list[0], "https://www.npmjs.com/package/express\r");
        assert_ne!(url_list[0], "");
        assert_ne!(url_list[0], "https://www.npmjs.com/package/vue\r");
        simple_log::info!("Read express url from file");

        assert_eq!(url_list[1], "https://www.npmjs.com/package/vue\r");
        assert_ne!(url_list[1], "");
        assert_ne!(url_list[1], "https://www.npmjs.com/package/react\r");
        simple_log::info!("Read vue url from file");

        assert_eq!(url_list[2], "https://www.npmjs.com/package/react\r");
        assert_ne!(url_list[2], "");
        assert_ne!(url_list[2], "https://www.npmjs.com/package/svelte\r");
        simple_log::info!("Read react url from file");

        assert_eq!(url_list[3], "https://www.npmjs.com/package/svelte\r");
        assert_ne!(url_list[3], "");
        assert_ne!(url_list[3], "https://www.npmjs.com/package/next");
        simple_log::info!("Read svelte url from file");

        assert_eq!(url_list[4], "https://www.npmjs.com/package/next");
        assert_ne!(url_list[4], "https://www.npmjs.com/package/next\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read next url from file");
        simple_log::info!("test URL 1 success");
        
    }

    #[test]
    fn test_get_urls_2() {
        simple_log::info!("test URL 2");
        let path = "./test/npm_urls_2.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 5);
        assert_eq!(url_list[0], "https://www.npmjs.com/package/axios\r");
        assert_ne!(url_list[0], "");
        assert_ne!(url_list[0], "https://www.npmjs.com/package/axios");
        simple_log::info!("Read axios url from file");

        assert_eq!(url_list[1], "https://www.npmjs.com/package/webpack\r");
        assert_ne!(url_list[1], "https://www.npmjs.com/package/webpack");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read webpack url from file");

        assert_eq!(url_list[2], "https://www.npmjs.com/package/lodash\r");
        assert_ne!(url_list[2], "https://www.npmjs.com/package/lodash");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read lodash url from file");

        assert_eq!(url_list[3], "https://www.npmjs.com/package/fastify\r");
        assert_ne!(url_list[3], "https://www.npmjs.com/package/fastify");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read fastify url from file");

        assert_eq!(url_list[4], "https://www.npmjs.com/package/async");
        assert_ne!(url_list[4], "https://www.npmjs.com/package/async\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read async url from file");

        simple_log::info!("test URL 2 success");
    }

    #[test]
    fn test_get_urls_3() {
        simple_log::info!("test URL 3");
        let path = "./test/npm_urls_3.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 5);
        assert_eq!(url_list[0], "https://www.npmjs.com/package/aws-sdk\r");
        assert_ne!(url_list[0], "https://www.npmjs.com/package/aws-sdk");
        assert_ne!(url_list[0], "");
        simple_log::info!("Read aws-sdk url from file");

        assert_eq!(url_list[1], "https://www.npmjs.com/package/bcrypt\r");
        assert_ne!(url_list[1], "https://www.npmjs.com/package/bcrypt");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read bcrypt url from file");

        assert_eq!(url_list[2], "https://www.npmjs.com/package/cors\r");
        assert_ne!(url_list[2], "https://www.npmjs.com/package/cors");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read cors url from file");

        assert_eq!(url_list[3], "https://www.npmjs.com/package/deep-equal\r");
        assert_ne!(url_list[3], "https://www.npmjs.com/package/deep-equal");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read deep-equal url from file");

        assert_eq!(url_list[4], "https://www.npmjs.com/package/eslint");
        assert_ne!(url_list[4], "https://www.npmjs.com/package/eslint\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read eslint url from file");

        simple_log::info!("test URL 3 success");
    }

    #[test]
    fn test_get_urls_4() {
        simple_log::info!("test URL 4");
        let path = "./test/bad_urls.txt";

        let url_list: Vec<String> = get_urls(&path);
        assert_eq!(url_list.len(), 10);
        assert_eq!(url_list[0], "https://github.com/phonegap/phonegap-app-anyconference");
        assert_ne!(url_list[0], "https://github.com/phonegap/phonegap-app-anyconference\r");
        assert_ne!(url_list[0], "");
        simple_log::info!("Read phonegap url from file");

        assert_eq!(url_list[1], "https://github.com/ReversedK/LocateAnything");
        assert_ne!(url_list[1], "https://github.com/ReversedK/LocateAnything\r");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read LocateAnything url from file");

        assert_eq!(url_list[2], "https://github.com/l3lackcurtains/graphql-boilerplate");
        assert_ne!(url_list[2], "https://github.com/l3lackcurtains/graphql-boilerplate\r");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read graphql-boilerplate url from file");

        assert_eq!(url_list[3], "https://github.com/vbaicu/mMusicCast");
        assert_ne!(url_list[3], "https://github.com/vbaicu/mMusicCast\r");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read mMusicCast url from file");

        assert_eq!(url_list[4], "https://github.com/anychart-solutions/anystock-drawing-tools-and-annotations-demo");
        assert_ne!(url_list[4], "https://github.com/anychart-solutions/anystock-drawing-tools-and-annotations-demo\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read anystock-drawing-tools-and-annotations-demo url from file");

        assert_eq!(url_list[5], "https://www.npmjs.com/package/url-inspector");
        assert_ne!(url_list[5], "https://www.npmjs.com/package/url-inspector\r");
        assert_ne!(url_list[5], "");
        simple_log::info!("Read url-inspector url from file");

        assert_eq!(url_list[6], "https://www.npmjs.com/package/sharebutton");
        assert_ne!(url_list[6], "https://www.npmjs.com/package/sharebutton\r");
        assert_ne!(url_list[6], "");
        simple_log::info!("Read sharebutton url from file");

        assert_eq!(url_list[7], "https://www.npmjs.com/package/anycontrol");
        assert_ne!(url_list[7], "https://www.npmjs.com/package/anycontrol\r");
        assert_ne!(url_list[7], "");
        simple_log::info!("Read anycontrol url from file");

        assert_eq!(url_list[8], "https://www.npmjs.com/package/pan-zoom");
        assert_ne!(url_list[8], "https://www.npmjs.com/package/pan-zoom\r");
        assert_ne!(url_list[8], "");
        simple_log::info!("Read pan-zoom url from file");

        assert_eq!(url_list[9], "https://www.npmjs.com/package/opentok-screen-sharing");
        assert_ne!(url_list[9], "https://www.npmjs.com/package/opentok-screen-sharing\r");
        assert_ne!(url_list[9], "");
        simple_log::info!("Read opentok-screen-sharing url from file");

        simple_log::info!("test URL 4 success");
    }

    #[test]
    fn test_get_urls_5() {
        simple_log::info!("test URL 5");
        let path = "./test/good_urls.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 10);
        assert_eq!(url_list[0], "https://github.com/ramda/ramda");
        assert_ne!(url_list[0], "https://github.com/ramda/ramda\r");
        assert_ne!(url_list[0], "");
        simple_log::info!("Read ramda url from file");

        assert_eq!(url_list[1], "https://github.com/debug-js/debug");
        assert_ne!(url_list[1], "https://github.com/debug-js/debug\r");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read debug url from file");

        assert_eq!(url_list[2], "https://github.com/josephg/ShareJS");
        assert_ne!(url_list[2], "https://github.com/josephg/ShareJS\r");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read ShareJS url from file");

        assert_eq!(url_list[3], "https://github.com/jashkenas/underscore");
        assert_ne!(url_list[3], "https://github.com/jashkenas/underscore\r");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read underscore url from file");

        assert_eq!(url_list[4], "https://github.com/Automattic/mongoose");
        assert_ne!(url_list[4], "https://github.com/Automattic/mongoose\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read mongoose url from file");

        assert_eq!(url_list[5], "https://www.npmjs.com/package/express");
        assert_ne!(url_list[5], "https://www.npmjs.com/package/express\r");
        assert_ne!(url_list[5], "");
        simple_log::info!("Read express url from file");

        assert_eq!(url_list[6], "https://www.npmjs.com/package/async");
        assert_ne!(url_list[6], "https://www.npmjs.com/package/async\r");
        assert_ne!(url_list[6], "");
        simple_log::info!("Read async url from file");

        assert_eq!(url_list[7], "https://www.npmjs.com/package/lodash");
        assert_ne!(url_list[7], "https://www.npmjs.com/package/lodash\r");
        assert_ne!(url_list[7], "");
        simple_log::info!("Read lodash url from file");

        assert_eq!(url_list[8], "https://www.npmjs.com/package/axios");
        assert_ne!(url_list[8], "https://www.npmjs.com/package/axios\r");
        assert_ne!(url_list[8], "");
        simple_log::info!("Read axios url from file");

        assert_eq!(url_list[9], "https://www.npmjs.com/package/mocha");
        assert_ne!(url_list[9], "https://www.npmjs.com/package/mocha\r");
        assert_ne!(url_list[9], "");
        simple_log::info!("Read mocha url from file");

        simple_log::info!("test URL 5 success");
    }

    #[test]
    fn test_get_urls_6() {
        simple_log::info!("test URL 6");
        let path = "./test/more_urls.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 10);
        assert_ne!(url_list[0], "https://github.com/googleapis/gapic-generator-php");
        assert_eq!(url_list[0], "https://github.com/googleapis/gapic-generator-php\r");
        assert_ne!(url_list[0], "");
        simple_log::info!("Read gapic url from file");

        assert_ne!(url_list[1], "https://github.com/michpolicht/CuteHMI");
        assert_eq!(url_list[1], "https://github.com/michpolicht/CuteHMI\r");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read CuteHMI url from file");

        assert_ne!(url_list[2], "https://github.com/czyt1988/SARibbon");
        assert_eq!(url_list[2], "https://github.com/czyt1988/SARibbon\r");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read SARibbon url from file");

        assert_ne!(url_list[3], "https://github.com/googleapis/python-documentai-toolbox");
        assert_eq!(url_list[3], "https://github.com/googleapis/python-documentai-toolbox\r");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read toolbox url from file");

        assert_ne!(url_list[4], "https://www.npmjs.com/package/uuid");
        assert_eq!(url_list[4], "https://www.npmjs.com/package/uuid\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read uuid url from file");

        assert_ne!(url_list[5], "https://www.npmjs.com/package/pactum");
        assert_eq!(url_list[5], "https://www.npmjs.com/package/pactum\r");
        assert_ne!(url_list[5], "");
        simple_log::info!("Read pactum url from file");

        assert_ne!(url_list[6], "https://www.npmjs.com/package/w-statistic");
        assert_eq!(url_list[6], "https://www.npmjs.com/package/w-statistic\r");
        assert_ne!(url_list[6], "");
        simple_log::info!("Read statistic url from file");

        assert_ne!(url_list[7], "https://www.npmjs.com/package/p-timeout");
        assert_eq!(url_list[7], "https://www.npmjs.com/package/p-timeout\r");
        assert_ne!(url_list[7], "");
        simple_log::info!("Read p-timeout url from file");

        assert_ne!(url_list[8], "https://www.npmjs.com/package/jsdoc");
        assert_eq!(url_list[8], "https://www.npmjs.com/package/jsdoc\r");
        assert_ne!(url_list[8], "");
        simple_log::info!("Read jsdoc url from file");

        assert_eq!(url_list[9], "https://www.npmjs.com/package/d-svg");
        assert_ne!(url_list[9], "https://www.npmjs.com/package/d-svg\r");
        assert_ne!(url_list[9], "");
        simple_log::info!("Read d-svg url from file");

        simple_log::info!("test URL 6 success");
    }

    #[test]
    fn test_get_urls_7() {
        simple_log::info!("test URL 7");
        let path = "./test/more_urls2.txt";
        let url_list: Vec<String> = get_urls(&path);

        assert_eq!(url_list.len(), 10);
        assert_ne!(url_list[0], "https://www.npmjs.com/package/z-schema");
        assert_eq!(url_list[0], "https://www.npmjs.com/package/z-schema\r");
        assert_ne!(url_list[0], "");
        simple_log::info!("Read z-schema url from file");

        assert_ne!(url_list[1], "https://www.npmjs.com/package/shelljs");
        assert_eq!(url_list[1], "https://www.npmjs.com/package/shelljs\r");
        assert_ne!(url_list[1], "");
        simple_log::info!("Read shelljs url from file");

        assert_ne!(url_list[2], "https://www.npmjs.com/package/knex");
        assert_eq!(url_list[2], "https://www.npmjs.com/package/knex\r");
        assert_ne!(url_list[2], "");
        simple_log::info!("Read knex url from file");

        assert_ne!(url_list[3], "https://www.npmjs.com/package/less");
        assert_eq!(url_list[3], "https://www.npmjs.com/package/less\r");
        assert_ne!(url_list[3], "");
        simple_log::info!("Read less url from file");

        assert_ne!(url_list[4], "https://www.npmjs.com/package/q");
        assert_eq!(url_list[4], "https://www.npmjs.com/package/q\r");
        assert_ne!(url_list[4], "");
        simple_log::info!("Read q url from file");

        assert_ne!(url_list[5], "https://www.npmjs.com/package/xml-js");
        assert_eq!(url_list[5], "https://www.npmjs.com/package/xml-js\r");
        assert_ne!(url_list[5], "");
        simple_log::info!("Read xml url from file");

        assert_ne!(url_list[6], "https://github.com/AlDanial/cloc");
        assert_eq!(url_list[6], "https://github.com/AlDanial/cloc\r");
        assert_ne!(url_list[6], "");
        simple_log::info!("Read cloc url from file");

        assert_ne!(url_list[7], "https://github.com/gventuri/pandas-ai");
        assert_eq!(url_list[7], "https://github.com/gventuri/pandas-ai\r");
        assert_ne!(url_list[7], "");
        simple_log::info!("Read pandas url from file");

        assert_ne!(url_list[8], "https://github.com/xtekky/gpt4free");
        assert_eq!(url_list[8], "https://github.com/xtekky/gpt4free\r");
        assert_ne!(url_list[8], "");
        simple_log::info!("Read gpt4 url from file");

        assert_eq!(url_list[9], "https://github.com/Ryujinx/Ryujinx");
        assert_ne!(url_list[9], "https://github.com/Ryujinx/Ryujinx\r");
        assert_ne!(url_list[9], "");
        simple_log::info!("Read ryu url from file");

        simple_log::info!("test URL 7 success");
    }

    #[test]
    fn test_empty_file() {
        simple_log::info!("test empty file");
        let path = "./test/empty_file.txt";
        let url_list: Vec<String> = get_urls(&path);
        assert_eq!(url_list.len(), 1);
        assert_eq!(url_list[0], "");
        simple_log::info!("Read empty file");
        simple_log::info!("test empty file success");
    }

    #[test]
    fn test_get_github_url_for_npm() {
        simple_log::info!("test get github url for npm");
        let npm_url = "https://www.npmjs.com/package/axios";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/axios/axios");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved axios github url");

        let npm_url = "https://www.npmjs.com/package/lodash";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/lodash/lodash");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved lodash github url");

        let npm_url = "https://www.npmjs.com/package/react";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/facebook/react");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved react github url");

        let npm_url = "https://www.npmjs.com/package/svelte";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/sveltejs/svelte");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved svelte github url");

        let npm_url = "https://www.npmjs.com/package/next";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/vercel/next.js");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved nextjs github url");

        let npm_url = "https://www.npmjs.com/package/express";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/expressjs/express");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved expressjs github url");

        let npm_url = "https://www.npmjs.com/package/vue";
        let github_url = get_github_url_for_npm(npm_url).unwrap();
        assert_eq!(github_url, "https://github.com/vuejs/core");
        assert_ne!(github_url, "");
        simple_log::info!("successfully retrieved vuejs github url");
    }

        
}