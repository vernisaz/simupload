use std::{io::{self,prelude::*,BufReader},
    fs::File, path::PathBuf, };

fn main() -> io::Result<()> {
    match std::env::var("PATH_INFO") {
        Err(_) => err_out("no path"),
        Ok(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.exists() && path_buf.is_file() {
                let md = path_buf.metadata().unwrap();
                let len = md.len();
                let name = path_buf.file_name().unwrap().display();
                print!("Content-Length: {len}\r\nContent-Type: application/octet-stream\r\nContent-Disposition: attachment; filename=\"{name}\"\r\n\r\n");
                let file = File::open(path_buf)?;
                let mut reader = BufReader::new(file);
                const BUFFER_LEN: usize = 4096;
                let mut buffer = [0u8; BUFFER_LEN];
                let mut stdout = io::stdout();
                loop {
                    let read_count = reader.read(&mut buffer)?;
                    if read_count == 0 {
                        break; // End of file
                    }
                    stdout.write_all(&buffer[0..read_count])?; // Writes the entire buffer to stdout.
                }
                stdout.flush()?; 
            }
        }
    }
    Ok(())
}

fn err_out(err: &str) {
    print!{ "Status: {} Internal Server Error\r\n", 500 }
    print!("Content-Length: {}\r\n", err.len());
    print! {"Content-type: text/plain\r\n\r\n{err}"}
}