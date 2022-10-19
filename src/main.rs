use clap::Parser;
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about, long_about = None)]
struct Args
{
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    format: String,
    #[arg(short, long)]
    outname: String,
    #[arg(short, long)]
    endformat: String,

}
#[derive(Deserialize, Serialize, Debug)]
struct Point
{
   x : u64,
   y : u64,
}

fn read_file(input_name_file: String, input_format : String) -> String
{
    let x = input_name_file.to_string() + &input_format.to_string();
    let mut file = File::open(x).expect("Error");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error");
    return data;
}

fn from_json(input_name_file: String, input_format : String) -> Point
{
    let data = read_file(input_name_file, input_format);
    let p: Point = serde_json::from_str(&data).expect("Error");
    return p;
}

fn from_yaml(input_name_file : String, input_format : String) -> Point
{
    let data = read_file(input_name_file,input_format);
    let p: Point = serde_yaml::from_str(&data).expect("Error");
    return p;
}

fn from_toml(input_name_file : String, input_format : String) -> Point
{
    let data = read_file(input_name_file, input_format);
    let p: Point = toml::from_str(&data).expect("Error");
    return p;
}

fn from_ron(input_name_file : String, input_format : String) -> Point
{
    let data = read_file(input_name_file, input_format);
    let p : Point = ron::from_str(&data).expect("Error");
    return p;
}

fn form_struct(input_name_file : String, input_format : String) -> Point
{
    let p : Point;
    match input_format.as_str()
    {
        ".json" => p = from_json(input_name_file, input_format),
        ".ron" => p = from_ron(input_name_file, input_format),
        ".yaml" => p = from_yaml(input_name_file, input_format),
        ".toml" => p = from_toml(input_name_file, input_format),
        _ => panic!("Error")
    }
    return p;
}
fn to_json(output_name_file : String, output_format : String, p : Point)
{
    let x = output_name_file.to_string() + &output_format.to_string();
    let mut file = File::create(x).expect("Error");
    let data = serde_json::to_string_pretty(&p).unwrap();
    file.write_all(data.as_bytes()).expect("Error");
}

fn to_ron(output_name_file : String, output_format : String, p : Point)
{
    let x = output_name_file.to_string() + &output_format.to_string();
    let mut file = File::create(x).expect("Error");
    let data = ron::to_string(&p).unwrap();
    file.write_all(data.as_bytes()).expect("Error");
}

fn to_yaml(output_name_file : String, output_format : String,  p : Point)
{
    let x = output_name_file.to_string() + &output_format.to_string();
    let mut file = File::create(x).expect("Error");
    let data = serde_yaml::to_string(&p).unwrap();
    file.write_all(data.as_bytes()).expect("Error");
}

fn to_toml(output_name_file : String, output_format : String, p : Point)
{
    let x = output_name_file.to_string() + &output_format.to_string();
    let mut file = File::create(x).expect("Error");
    let data = toml::to_string(&p).unwrap();
    file.write_all(data.as_bytes()).expect("Error");
}

fn print_to_file(output_name_file : String, output_format : String, p : Point)
{
    match output_format.as_str()
    {
        ".json" => to_json(output_name_file, output_format, p),
        ".ron" => to_ron(output_name_file, output_format, p),
        ".toml" => to_toml(output_name_file, output_format, p),
        ".yaml" => to_yaml(output_name_file, output_format, p),
        _ => panic!("Error")
    }
}

fn convert(input_name_file : String, input_format : String, output_name_file : String, output_format : String)
{
    let p = form_struct(input_name_file, input_format);
    print_to_file(output_name_file, output_format, p);
}

fn main()
{
    let args = Args::parse();
    let input_name_file: String = args.name;
    let input_format: String = args.format;
    let output_name_file: String = args.outname;
    let output_format: String = args.endformat;
    convert(input_name_file, input_format, output_name_file, output_format);
}