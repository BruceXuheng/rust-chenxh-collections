use std::fs;

fn main() {
    let url = "https://www.baidu.com";
    let contents = get_url(url);

    let output = "baidu.html";

    fs::write(output, contents).unwrap();

}

fn get_url(url:&str)->String{
    return reqwest::blocking::get(url).unwrap().text().unwrap()
}