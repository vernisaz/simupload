extern crate simjson;
extern crate simweb;
use std::{io::{self}, fmt::Write, fs, path::PathBuf, ffi::OsStr,};
use simweb::{WebData,WebPage};
struct Upload;

fn main() -> io::Result<()> {
    Ok(Upload{}.show())
}

impl simweb::WebPage for Upload {
    fn main_load(&self) -> Result<String, String> {
        let data = WebData::new();
        if data.param("dir").is_some() {
            let dir = data.param("dir").unwrap();
            match data.params("upFile") {
                None => Err("Err: no files".to_string()),
                Some(files) => {
                    let mut errors: Option<String> = None;
                    for file in files {
                        let file_path = PathBuf::from(&file);
                        let mut dest = PathBuf::from(&dir);
                        dest.push(file_path.file_name().unwrap_or(OsStr::new("")));
                        match fs::rename(&file,&dest) {
                            Ok(()) => (),
                            Err(err) => match errors {
                                None => {
                                    let mut some_errors = String::new();
                                    writeln!{&mut some_errors, "{file} to {dir} {err:?}"}.unwrap();
                                    errors = Some(some_errors)
                                }
                                Some(mut some_errors) => {
                                    writeln!{&mut some_errors, "{file} to {dir} {err:?}"}.unwrap();
                                    errors = Some(some_errors)
                                }
                            }
                        }
                    }
                    match errors {
                        None => Ok("Ok".to_string()),
                        Some(errors) => Ok(format!("Err: {errors}"))
                    }
                }
            }
        } else {
          let mut res = String::from(r#"<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Form values</title>
    <style>
        :root {
          color-scheme: light dark;
        }
        body {
          color: light-dark(#333b3c, #efefec);
          background-color: light-dark(#efedea, #223a2c);
        }
    </style>
</head>
<body><h2>A multi-part form data</h2>"#);
            print_param(&mut res, "name", data.param("name")).unwrap();
            match data.param("name") {
                None => write!{&mut res, "Oh, something wrong"}.unwrap(),
                Some(name) if name == "boxes" => {
                    print_params(&mut res, "автомобили", data.params("vehicle")).unwrap();
                }
                _ => {
                    print_param(&mut res, "browsefile", data.param("browsefile")).unwrap();
                    print_param(&mut res, "somedata", data.param("somedata")).unwrap();
                    
                    print_param(&mut res, "russo", data.param("russo")).unwrap(); 
                }
            }
    
            write!{&mut res, "</body></html>"}.unwrap();
            Ok(res)
        }
    }
}

fn print_param( accumulation: &mut String, name: &str, val: Option<String>) -> Result<(), std::fmt::Error> {
    write!{accumulation, "<div> parameter F˚– <b>{name}</b> of <i>{val:?}</i></div>"}
}

fn print_params(accumulation: &mut String, name: &str, val: Option<Vec<String>>) -> Result<(), std::fmt::Error> {
    write!{accumulation, "<div> parameter <b>{name}</b> of <i>{val:?}</i></div>"}
}