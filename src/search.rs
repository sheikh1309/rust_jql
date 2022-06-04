extern crate colored_json;
use crate::search_command_builder::SearchOptions;
use std::{error::Error, io::stdin};
use std::io::{Read, stdout, Write};
use serde_json::Value;
use serde_json::json;

pub fn piped_search(options: SearchOptions) -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let _lines = stdin().read_to_string(& mut buffer);
    let json: Value = serde_json::from_str(buffer.as_str())?;
    if !options.query.starts_with('.') {
        Err("Query Must start with '.'")?;
    }
    let json_result = get_search_result(options, json).unwrap();
    print_json(&json_result)?;
    Ok(())

}

pub fn in_file_search(options: SearchOptions, file_content: String) -> Result<(), Box<dyn Error>> {
    let json: Value = serde_json::from_str(file_content.as_str())?;
    if !options.query.starts_with('.') {
        Err("Query Must start with '.'")?;
    }
    let json_result = get_search_result(options, json).unwrap();
    print_json(&json_result)?;
    Ok(())

}

fn get_search_result(options: SearchOptions, json: Value) -> Result<Value, Box<dyn Error>> {
    let mut json_result = json;
    let keys =  &options.query[1..options.query.len()].split(".").collect::<Vec<&str>>();
    for (index, key) in keys.iter().enumerate() {
        if index + 1 == keys.len() {
            json_result = parse_last_key(key, json_result)
        } else {
            json_result = parse_search_query(key, json_result);
        }
    }
    Ok(json_result)
}

fn parse_search_query(key: &str, json_result: Value) -> Value {
    if key == "[]" {
        parse_array_key(json_result).unwrap()
    } else {
        parse_object_key(key, json_result)
    }
}

fn parse_object_key(key: &str, json_result: Value) -> Value {
    if json_result.is_array() {
        let mut elements: Vec<&Value> = vec![];
        for value in json_result.as_array().unwrap() {
            let data = value.get(key).unwrap();
            if data.is_array() {
                for val in data.as_array().unwrap() {
                    elements.push(val);
                }
            } else {
                elements.push(data);
            }
        }
        json!(elements)
    } else  {
        let obj = json_result.as_object().unwrap();
        json!(obj.get(key).unwrap())
    }
}

fn parse_array_key(json_result: Value) -> Result<Value, Box<dyn Error>> {
    if json_result.is_array() {
        return Ok(json!(json_result.as_array().unwrap()));
    }
    Err("Request [] in valid position")?
}

fn parse_last_key(key: &str, json_result: Value) -> Value {
    if json_result.is_array() {
        let vector = json_result.as_array().unwrap().into_iter().map(|v| v.get(key).unwrap()).collect::<Vec<_>>();
        json!(vector)
    }
    else {
        json!(json_result.as_object().unwrap().get(key))
    }
}

fn print_json(json: &Value) -> Result<(), Box<dyn Error>> {
    let out = stdout();
    {
        let mut out = out.lock();
        colored_json::write_colored_json(json, &mut out)?;
        out.flush()?;
        println!()
    }
    Ok(())
}