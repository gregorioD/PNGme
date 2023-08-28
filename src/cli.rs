use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use clap::{Parser, Subcommand};
use std::path::Path;
use std::str::FromStr;

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
                Cli::encode(
                    file_path.clone(),
                    chunk_type.clone(),
                    message.clone(),
                    output_file.clone(),
                );
            }
            Some(Commands::Decode {
                file_path,
                chunk_type,
            }) => {
                Cli::decode(file_path.clone(), chunk_type.clone());
            }
            Some(Commands::Remove {
                file_path,
                chunk_type,
            }) => {
                Cli::remove(file_path.clone(), chunk_type.clone());
            }
            Some(Commands::Print { file_path }) => {
                Cli::print_chunks(file_path.clone());
            }
            None => {
                println!("No subcommand provided.");
            }
        }
    }

    fn encode(
        file_path_str: String,
        chunk_type_str: String,
        message: String,
        output_file_str: Option<String>,
    ) {
        let file_path = Path::new(&file_path_str);
        let chunk_type = ChunkType::from_str(&chunk_type_str[..]).unwrap();

        let file_bytes = std::fs::read(&file_path).unwrap();
        let vec_message: Vec<u8> = Vec::from(message);

        let new_chunk = Chunk::new(chunk_type, vec_message);

        let mut png = Png::try_from(&file_bytes[..]).unwrap();

        png.append_chunk(new_chunk);

        match output_file_str {
            Some(o) => {
                let output_file_path = Path::new(&o);
                let png_bytes = png.as_bytes();
                std::fs::write(output_file_path, png_bytes).unwrap();
            }
            None => (),
        }
    }

    fn decode(file_path_str: String, chunk_type_str: String) -> Option<String> {
        let file_path = Path::new(&file_path_str);

        let file_bytes = std::fs::read(&file_path).unwrap();
        let png = Png::try_from(&file_bytes[..]).unwrap();

        match png.chunk_by_type(&chunk_type_str[..]) {
            Some(chunk) => Some(chunk.data_as_string().unwrap()),
            None => None,
        }
    }

    fn remove(file_path_str: String, chunk_type_str: String) {
        let file_path = Path::new(&file_path_str);

        let file_bytes = std::fs::read(&file_path).unwrap();
        let mut png = Png::try_from(&file_bytes[..]).unwrap();

        match png.remove_chunk(&chunk_type_str[..]) {
            Ok(_) => match std::fs::write(file_path, png.as_bytes()) {
                Err(e) => eprintln!("ERROR: {e}"),
                Ok(_) => (),
            },
            Err(e) => eprintln!("ERROR: {e}"),
        }
    }

    fn print_chunks(file_path_str: String) {
        let file_path = Path::new(&file_path_str);
        let file_bytes = std::fs::read(&file_path).unwrap();
        let png = Png::try_from(&file_bytes[..]).unwrap();

        for chunk in png.chunks() {
            let messages = chunk.data_as_string().unwrap();
            println!("{messages}");
        }
    }
}
