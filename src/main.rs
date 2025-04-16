use clap::Parser;
use toml_edit::{Datetime, DocumentMut};
use std::fs;
use std::error::Error;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    inputs: Vec<String>,
}

struct ResourceConfigStruct {
    rc_name: String,
    rc_description: String,
    rc_type: String,
    rc_url: String,
    rc_size:i16,
    rc_method: String,
    rc_author: String,
    rc_creation_date: Datetime,
    rc_last_updated_by: String,
    rc_last_updated_date: Datetime,
    rc_status: String,
    rc_version: String,
    rc_license: String,
    rc_access_level: String,
    rc_thumbnail_url: String,
    rc_checksum: String,
    rc_file_format: String,
    rc_metadata: String,
    rc_tags: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let resource_config_path: &str= &args.inputs[0].to_string();
    let resource_config_data:Result<DocumentMut, Box<dyn Error>> = get_resource_config(&resource_config_path);
    if resource_config_data.is_err() {
        println!("Error: {:?}", resource_config_data);
        return;
    }
    let geted_resource_config_data: DocumentMut = resource_config_data.unwrap().to_string().parse::<DocumentMut>().expect("Error parsing resource config data");
    let resource_config_data_parsed: ResourceConfigStruct = ResourceConfigStruct {
        rc_name: geted_resource_config_data["test-api"]["name"].to_string(),
        rc_description: geted_resource_config_data["test-api"]["description"].to_string(),
        rc_type: geted_resource_config_data["test-api"]["type"].to_string(),
        rc_url: geted_resource_config_data["test-api"]["url"].to_string(),
        rc_size: geted_resource_config_data["test-api"]["size"].as_integer().unwrap_or(0) as i16,
        rc_method: geted_resource_config_data["test-api"]["method"].to_string(),
        rc_author: geted_resource_config_data["test-api"]["author"].to_string(),
        rc_creation_date: geted_resource_config_data["test-api"]["creation_date"]
            .as_datetime()
            .unwrap()
            .clone(),
        rc_last_updated_by: geted_resource_config_data["test-api"]["last_updated_by"].to_string(),
        rc_last_updated_date: geted_resource_config_data["test-api"]["last_updated_date"]
            .as_datetime()
            .unwrap()
            .clone(),
        rc_status: geted_resource_config_data["test-api"]["status"].to_string(),
        rc_version: geted_resource_config_data["test-api"]["version"].to_string(),
        rc_license: geted_resource_config_data["test-api"]["license"].to_string(),
        rc_access_level: geted_resource_config_data["test-api"]["access_level"].to_string(),
        rc_thumbnail_url: geted_resource_config_data["test-api"]["thumbnail_url"].to_string(),
        rc_checksum: geted_resource_config_data["test-api"]["checksum"].to_string(),
        rc_file_format: geted_resource_config_data["test-api"]["file_format"].to_string(),
        rc_metadata: geted_resource_config_data["test-api"]["metadata"].to_string(),
        rc_tags: geted_resource_config_data["test-api"]["tags"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tag| tag.to_string())
            .collect(),
    };
    println!(
        "Resource Config:\n\
        Name: {:?}\n\
        Description: {:?}\n\
        Type: {:?}\n\
        URL: {:?}\n\
        Size: {:?}\n\
        Method: {:?}\n\
        Author: {:?}\n\
        Creation Date: {:?}\n\
        Last Updated By: {:?}\n\
        Last Updated Date: {:?}\n\
        Status: {:?}\n\
        Version: {:?}\n\
        License: {:?}\n\
        Access Level: {:?}\n\
        Thumbnail URL: {:?}\n\
        Checksum: {:?}\n\
        File Format: {:?}\n\
        Metadata: {:?}\n\
        Tags: {:?}",
        resource_config_data_parsed.rc_name,
        resource_config_data_parsed.rc_description,
        resource_config_data_parsed.rc_type,
        resource_config_data_parsed.rc_url,
        resource_config_data_parsed.rc_size,
        resource_config_data_parsed.rc_method,
        resource_config_data_parsed.rc_author,
        resource_config_data_parsed.rc_creation_date,
        resource_config_data_parsed.rc_last_updated_by,
        resource_config_data_parsed.rc_last_updated_date,
        resource_config_data_parsed.rc_status,
        resource_config_data_parsed.rc_version,
        resource_config_data_parsed.rc_license,
        resource_config_data_parsed.rc_access_level,
        resource_config_data_parsed.rc_thumbnail_url,
        resource_config_data_parsed.rc_checksum,
        resource_config_data_parsed.rc_file_format,
        resource_config_data_parsed.rc_metadata,
        resource_config_data_parsed.rc_tags
    );

}

fn get_resource_config(path: &str) -> Result<DocumentMut, Box<dyn Error>> {
    let def_cmd: String = fs::read_to_string(path)?;
    let def_cmd: DocumentMut = def_cmd.parse::<DocumentMut>()?;
    return Ok(def_cmd);
}