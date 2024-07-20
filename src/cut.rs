use std::io::{self, Read};
use std::io::IsTerminal;
use std::fs;
use std::process;
use carrot_libs::args;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    let mut i = 0;
    let mut separator = String::from(" ");
    let mut replace = None;
    let mut step = None;
    while i < swcs.len() {
        let s = &swcs[i];
        let v = &vals[i];

        if s != "s" && s != "sep" && s != "separator"
        && s != "r" && s != "rep" && s != "replace"
        && s != "x" && s != "step" 
        {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        // Make sure, that value is set!
        if v.is_empty() { 
            eprintln!("This switch requires a value: {s}!"); process::exit(1); 
        }

        if s == "s" || s == "sep" || s == "separator" {
            separator=v.clone();
        }
        if s == "r" || s == "rep" || s == "replace" {
            replace=Some(v.clone());
        }
        if s == "x" || s == "step" {
            // Step value can't be negative
            step = match v.parse::<usize>() {
                Err(e) => {
                    eprintln!("Can't parse a value for a switch: {s}={v}: {:?}!", e.kind());
                    process::exit(1);
                },
                Ok(res) => Some(res),
            };
        }
        i+=1;
    }

    if step.is_none() && replace.is_none() {
        eprintln!("You have to define a separator or use a step value to separate text!");
        process::exit(1);
    }
    
    // Show error when there are no files requested as options by user and nothing is piped to the program
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("Type the name of elements to use or use a pipe!");
        process::exit(1);
    }

    // If something is piped to our program, show it.
    if !io::stdin().is_terminal() {
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to get contents of stdin!");
        do_the_magic(contents_of_stdin, &separator, &replace, step);
    }

    if opts.is_empty() {
        process::exit(0);
    }
    // Show stuff requested as options
    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => {
                do_the_magic(f, &separator, &replace, step);
            },
        };
        index += 1;
    };
}

fn do_the_magic(text:String, separator:&String, replace:&Option<String>, step:Option<usize>) {
    println!("Doing the magic for the text: {}", text);
}
