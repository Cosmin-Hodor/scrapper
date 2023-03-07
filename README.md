## A basic scrapper written in Rust

This code uses the hyper and regex crates to scrape links from a web page. The code first creates an instance of the Client struct from the hyper crate, which is used to make HTTP requests, then prompts the user to enter a URL, reads the URL from the user input using stdin, and stores it in the url variable. The newline character is trimmed from the end of the URL string using the trim method, then uses the get method from the Client instance to make a GET request to the specified URL. 

The response from the server is stored in the response variable. 
The body of the response is read into the body variable, which is a string.

The code then uses the regex crate to define a regular expression pattern that matches links in the HTML body. The Regex::new function is used to compile the pattern, which looks for <a> elements with a href attribute. The captures_iter method is used to iterate over all the links that match the pattern in the body string. For each match, the link is extracted from the first capture group and printed to the console using println!.
