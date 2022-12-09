extern crate hyper;
extern crate regex;

use hyper::client::Client;
use std::io::Read;
use regex::Regex;

fn main(url: String) {
  let client = Client::new();

  // Retrieves page
  let mut response = client.get(url).send().unwrap();

  // Initiate storage for our page
  let mut body = String::new();

  // Store our page's body
  response.read_to_string(&mut body).unwrap();

  // We define a regex pattern to scrape links from our page
  let re = Regex::new(r#"<a[^>]*href="([^"]*)""#).unwrap();

  // Iterate over our link vector
  for cap in re.captures_iter(&body) {
    let link = &cap[1];
    
    // Log each link to console
    println!("Link: {}", link);
  };
};