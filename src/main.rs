extern crate regex;
extern crate walkdir;

use regex::Regex;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let mut directory_path = String::new();

    println!("Enter the directory path: ");
    io::stdin()
        .read_line(&mut directory_path)
        .expect("Failed to read input");

    let directory_path = directory_path.trim().replace("\"", ""); // Remove newline characters

    let docx_regex = Regex::new(r"^.+\.(docx|xlsx)$").unwrap();
    let pdf_regex = Regex::new(r"^.+\.pdf$").unwrap();

    let mut approved_documents = vec![];
    let mut pending_documents = vec![];

    for entry in WalkDir::new(&directory_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let n = entry.path().to_str().unwrap();
            !n.contains("_Archive") && !n.contains("~$")
        })
    {
        if let Some(file_name) = entry.file_name().to_str() {
            if docx_regex.is_match(file_name) {
                let base_name = Path::new(file_name).file_stem().unwrap();
                let pdf_name = format!("{}.pdf", base_name.to_str().unwrap());
                let pdf_path = entry.path().with_file_name(pdf_name.clone());

                if pdf_path.exists() && pdf_regex.is_match(&pdf_name) {
                    approved_documents.push(file_name.to_string());
                } else {
                    pending_documents.push(file_name.to_string());
                }
            }
        }
    }

    println!("Approved Documents:");
    for doc in approved_documents {
        println!("{}", doc);
    }

    println!("\nPending Documents:");
    for doc in pending_documents {
        println!("{}", doc);
    }
}
