use std::fs::{self, File};
use std::io::prelude::*;

pub fn read(path: &str, item: &str, get: &str) -> String{
    
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error reading \"{path}\"\n ↳ Reason: {e}");
            return "".to_string();
        },
    };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reading \"{path}\"\n ↳ Reason: {e}");
        return "".to_string();
    }
    
    let mut content_data: Vec<String> = Vec::new();
    let mut content_content: Vec<String> = Vec::new();
    let mut raw = String::new();
    let mut record = false;
    for i in contents.lines(){
        if record{
            if i.chars().next() == Some('[')
            && i.contains("]"){
                break;
            }
            content_content.push(i.to_string());
            raw.push('\n');
            raw.push_str(i);
        } else {
            if i.contains(format!("[{}]",item).as_str())
            && i.chars().next() == Some('['){
                raw.push_str(i);
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
                if content_data[index].starts_with("--ignorestarts:"){
                    let ign = content_data[index].replace("--ignorestarts:", "");
                    for i in content_content.clone(){
                        if !i.starts_with(ign.as_str()){
                            content_temp.push(i)
                        }
                    }
                    content_content.clear();
                    content_content = content_temp.clone();
                    content_temp.clear();
                } else if content_data[index].starts_with("--limit:"){
                    let limit: usize = content_data[index].replace("--limit:", "").parse().expect("msg");
                    let mut index = 0;
                    while index < limit {
                        content_temp.push(content_content[index].clone());
                        index += 1;
                    }
                    content_content.clear();
                    content_content = content_temp.clone();
                    content_temp.clear();
                }
                index += 1;
            }
        }
    }
    
    match get {
        "value" => {
            if raw.is_empty(){println!("Can't find \"{item}\"");}
            return content_content.join("\n");
        }
        "data" => {
            if raw.is_empty(){println!("Can't find \"{item}\"");}
            return content_data.join(" ");
        }
        "raw" => {
            if raw.is_empty(){println!("Can't find \"{item}\"");}
            return raw;
        }
        _ => {
            println!("Invalid output method \"{get}\"\n ↳ Fix: Returned Value");
            return content_content.join("\n");
        }
    }
}

pub fn remove(path: &str, item: &str) {
    
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error reading \"{path}\"\n ↳ Reason: {e}");
            return;
        }
    };
    
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reading \"{path}\"\n ↳ Reason: {e}");
    }
    
    let to_remove = format!("{}\n",read(path, item, "raw"));
    let contents = contents.replace(to_remove.as_str(), "");
    
    if !to_remove.trim().is_empty(){
        match fs::write(path, contents) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing in \"{path}\"\n ↳ Reason: {e}");
            }
        }
    } else {
        println!("Can't remove \"{item}\"")
    }
}

pub fn add(path: &str, item: &str, param: &str, value: &str){
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error reading \"{path}\"\n ↳ Reason: {e}");
            return;
        }
    };
    

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        println!("Error reading \"{path}\"\n ↳ Reason: {e}");
    }
    
    
}