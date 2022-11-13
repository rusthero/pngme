use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Parser, Subcommand};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod chunk;
mod chunk_type;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/// Hide secret messages in PNG files.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
enum Commands {
    /// Encode the secret message in the chunk.
    Encode {
        // Path of target PNG file to encode the secret message
        file: PathBuf,
        chunk_type: String,
        message: String,
    },

    /// Decode the secret message in the chunk.
    Decode {
        // Path of target PNG file to decode the secret message
        file: PathBuf,
        chunk_type: String,
    },

    /// Remove a chunk by its type.
    Remove {
        // Path of target PNG file to remove chunk
        file: PathBuf,
        chunk_type: String,
    },

    /// Print all chunks inside the PNG file.
    Print {
        /// Path of target PNG file to print its contents
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encode {
            file,
            chunk_type,
            message,
        }) => {
            let mut png = Png::from(file);

            png.append_chunk(Chunk::new(
                ChunkType::from_str(chunk_type).expect("cannot create chunk"),
                message.clone().into_bytes(),
            ));

            println!(
                "{}",
                fs::write(file, png.as_bytes()).map_or_else(
                    |_| "Successfully added secret message!",
                    |_| "Cannot add secret message",
                )
            );
        }

        Some(Commands::Decode { file, chunk_type }) => {
            let png = Png::from(file);
            let chunk = png
                .chunk_by_type(chunk_type)
                .unwrap_or_else(|| panic!("Chunk with type {} does not exist.", chunk_type));

            println!(
                "{}",
                chunk.data_as_string().expect("data is not an utf-8 string")
            );
        }

        Some(Commands::Remove { file, chunk_type }) => {
            let removed_chunk = Png::from(file)
                .remove_chunk(chunk_type)
                .expect("Cannot remove chunk.");

            println!("Chunk {} is successfully removed!", removed_chunk.r#type);
        }

        Some(Commands::Print { file }) => {
            println!(
                "{}",
                Png::from(file)
                    .chunks
                    .into_iter()
                    .map(|chunk| chunk.r#type.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        None => {
            unreachable!();
        }
    };
}
