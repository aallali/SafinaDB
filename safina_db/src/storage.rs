use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::kv_store::KV;

#[derive(Debug)]
pub struct Storage {
    file_path: Option<String>,
    pub file: Option<File>,
}

impl Storage {
    /// Creates a new Storage instance with an optional file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional string slice that holds the file path.
    ///
    /// # Returns
    ///
    /// * A new instance of Storage.
    pub fn new(file_path: Option<&str>) -> Self {
        Storage {
            file_path: file_path.map(|path| path.to_string()), // Convert the file path to a String and store it in the struct
            file: None, // Initialize the file as None
        }
    }

    /// Loads the file from the specified or existing file path and deserializes the content.
    ///
    /// # Arguments
    ///
    /// * `file_path` - An optional string slice that holds the file path.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<KV>)` - A vector of deserialized KV structs if the operation is successful.
    /// * `Box<dyn Error>>` - An error message if the operation fails.
    pub fn load_file(&mut self, file_path: Option<&str>) -> Result<Vec<KV>, Box<dyn Error>> {
        match file_path {
            Some(path) => {
                self.file_path = Some(path.to_string()); // Set the file path if provided
            }
            None => {
                if self.file_path.is_none() {
                    return Err("No file path set.".into()); 
                }
            }
        }

        let path = self.file_path.as_ref().unwrap(); // Safely unwrap the file path
        self.file = Some(
            OpenOptions::new()
                .read(true) // Open the file for reading
                .write(true) // Open the file for writing
                .create(true) // Create the file if it doesn't exist
                .open(path)? // Open the file at the specified path
        );

        if let Some(ref mut file) = self.file {
            let mut buffer = Vec::new(); // Create a buffer to store file contents
            // file.seek(SeekFrom::Start(0))?; // TODO :doc
            file.read_to_end(&mut buffer)?; // Read the file content into the buffer
            if buffer.is_empty() {
                return Ok(Vec::new())
            }
            let data: Vec<KV> = bincode::deserialize(&buffer)?;
            return Ok(data); // Return the deserialized data
        } else {
           return Err("No file open to read data.".into())
        }
    }

    /// Saves the provided data to the file by serializing it into binary format.
    ///
    /// # Arguments
    ///
    /// * `data` - A vector of KV structs to be serialized and saved.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the operation is successful.
    /// * `Err(Box<dyn Error>)` - An error message if the operation fails.
    pub fn save_file(&mut self, data: Vec<KV>) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut file) = self.file {
            let buffer: Vec<u8> = bincode::serialize(&data)?; // Serialize the data into a binary buffer
            file.set_len(0)?; // Clear the content of the file
            file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file
            file.write_all(&buffer)?; // Write the binary buffer to the file
            file.sync_all()?; // Sync all changes to the file
            Ok(())
        } else {
            Err("No file open to save data.".into()) // Return an error if no file is open
        }
    }
}