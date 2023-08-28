use clap::{Parser, Subcommand};
// use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: Option<String>,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove {
        file_path: String,
        chunk_type: String,
    },
    Print {
        file_path: String,
    },
}

impl Cli {
    pub fn run(&self) {
        // Use match on the reference to the enum variant
        match &self.command {
            Some(Commands::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            }) => {
                println!("Encode:");
                println!("File Path: {}", file_path);
                println!("Chunk Type: {}", chunk_type);
                println!("Message: {}", message);
                if let Some(output) = output_file {
                    println!("Output File: {}", output);
                }
            }
            Some(Commands::Decode {
                file_path,
                chunk_type,
            }) => {
                println!("Decode:");
                println!("File Path: {}", file_path);
                println!("Chunk Type: {}", chunk_type);
            }
            Some(Commands::Remove {
                file_path,
                chunk_type,
            }) => {
                println!("Remove:");
                println!("File Path: {}", file_path);
                println!("Chunk Type: {}", chunk_type);
            }
            Some(Commands::Print { file_path }) => {
                println!("Print:");
                println!("File Path: {}", file_path);
            }
            None => {
                println!("No subcommand provided.");
            }
        }
    }
}
