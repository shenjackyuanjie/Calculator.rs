use std::fs::File;
use std::io::{self, BufRead};

use crate::exec::attempt::attempt;
use crate::public::run_time::scope::Scope;

use super::pre_processer;

type FileBuf = io::BufReader<File>;
fn read_lines(path: String) -> io::Result<io::Lines<FileBuf>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run_script(
    path: String,
    scope: &mut Scope
) {
    let Ok(mut script_lines) = read_lines(path) else {
        println!("Invalid script file.");
        return
    };

    let mut cached_multiline = String::new();
    let mut line_count = 0;
    loop {
        match script_lines.next() {
            Some(item) => {
                line_count += 1;

                let mut script_line =
                if let Ok(line) = item {
                    pre_processer::process(line)
                } else {
                    String::new()
                };

                // multi-line symbol: `:`
                if script_line.ends_with(":") {
                    script_line.pop();
                    cached_multiline += &script_line;
                } else {
                    // out of multi-line statement
                    // or last line of multi-line statement
                    let current_line: &String;

                    if cached_multiline.is_empty() {
                        // out of multi-line statement
                        script_line += "\r\n";
                        current_line = &script_line;
                    } else {
                        // the last line of multi-line statement
                        // or the blank line || line comment
                        if script_line.len() == 0 {
                            // skip the blank line and line comment
                            continue;
                        }
                        // add reture sign to current line
                        script_line += "\r\n";

                        cached_multiline += &script_line;
                        current_line = &cached_multiline;
                    }
                    // execuse the line
                    let line_result =
                        attempt(current_line, scope);

                    if line_result.is_err() {
                        println!("Error occured at line {}.", line_count);

                        // print error code
                        println!("Code: `{}`.", current_line);
                        break;
                    }

                    if !cached_multiline.is_empty() {
                        cached_multiline.clear();
                    }
                }
            },
            // if is the last line
            None => break,
        }
    }
}