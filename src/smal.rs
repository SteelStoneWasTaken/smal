use std::fs::File;
use std::io::prelude::*;

pub fn read(path: &str, item: &str, get: &str) -> String{
    
    let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                println!("Error reading file.");
                return "".to_string();
            },
        };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
            return format!("Erro ao ler o arquivo: {}", e);
    }
    
    let mut content_data: Vec<String> = Vec::new();
    let mut content_content: Vec<String> = Vec::new();
    let mut record = false;
    for i in contents.lines(){
        if record{
            if i.chars().next() == Some('[')
            && i.contains("]"){
                break;
            }
            content_content.push(i.to_string());
        } else {
            if i.contains(format!("[{}]",item).as_str())
            && i.chars().next() == Some('['){
                content_data.push(item.to_string());
                let i = i.replace(format!("[{}]", item).as_str(), "").trim().to_string();
                
                if !i.is_empty(){
                    for i in i.split_whitespace() {
                        content_data.push(i.to_string())
                    }
                }
                record = true;
            }
        }
    }
    
    let mut content_temp: Vec<String> = Vec::new();
    let mut index = 1;
    while index < content_data.len(){
        match content_data[index].as_str() {
            "--ignorewhitespaces" => {
                for i in content_content.clone(){
                    if !i.is_empty(){
                        content_temp.push(i)
                    }
                }
                content_content.clear();
                content_content = content_temp.clone();
                content_temp.clear();
                
                index += 1;
            }
            _ => {
                if content_data[index].starts_with("--allowcommentswith:"){
                    let ign = content_data[index].replace("--allowcommentswith:", "");
                    for i in content_content.clone(){
                        if !i.starts_with(ign.as_str()){
                            content_temp.push(i)
                        }
                    }
                    content_content.clear();
                    content_content = content_temp.clone();
                    content_temp.clear();
                }
                index += 1;
            }
        }
    }
    
    println!("{content_data:?}\n{content_content:?}");
    match get {
        "value" => {
            return content_content.join("");
        }
        "data" => {
            return content_content.join(" / ");
        }
        _ => {
            println!("value");
            return content_content.join("");
        }
    }
}