#![feature(path_file_prefix)]
#![feature(buf_read_has_data_left)]
#![feature(file_create_new)]

use std::{
    env,
    fs::{File},
    io::{self, BufRead, Read, Write},
    hash::Hash,
    path::{Path, PathBuf}
};

use serde::{Serialize, Deserialize};

pub mod decode;
pub mod encode;

#[derive(Debug)]
pub struct FilePart {
    pub part_file: File,
    pub hash_bytes: Vec<u8>,
    pub part_file_name: String,
}

#[derive(Debug)]
pub struct CompositeFile {
    pub filename: String,
    pub file_extension: String,
    pub file_len: usize,
    pub parts: Vec<FilePart>,
    pub uuid_parts: String,
}

#[derive(Debug)]
pub struct Config {
    pub method: char,
    pub path: PathBuf,
    pub options: Options
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Options {
	pub path_for_save: Option<PathBuf>,
    pub count_parts: Option<u8>,
    pub part_size: Option<usize>,
	pub compressed: Option<bool>,
}

impl Config {

    pub fn new(method: char, path: &str, options: Options) -> Self {
        Config {
			method,
            path: PathBuf::from(path),
            options,
        }
    }

    pub fn new_from_env_args() -> Self {
        let mut args_peekable = env::args().peekable();

        let mut config: Option<Config> = None;

        while let Some(arg) = args_peekable.next() {
            match arg.as_str() {
                "-e" => {
                    if let Some(path) = args_peekable.peek() {
                        config = Some(Config::new('e', path, Options::default()));
                    }
                },
                "-d" => {
                    if let Some(path) = args_peekable.peek() {
                        config = Some(Config::new('d', path, Options::default()));
                    }
                },
                "-s" => {
                    if let Some(part_size) = args_peekable.peek() {
                        let part_size = part_size.parse::<usize>().unwrap();

                        match config.as_mut() {

                            Some(config) =>
                                config.options.part_size = Some(part_size),

                            None => {}
                        }
                    }
                },
				"-n" => {
					if let Some(count_parts) = args_peekable.peek() {
						let count_parts = count_parts.parse::<u8>().unwrap();
					}
				},
				"-save_path" => {
					if let Some(path_for_save) = args_peekable.peek() {

                        match config.as_mut() {

                            Some(config) =>
                                config.options.path_for_save = Some(path_for_save.into()),

                            None => {}
                        }
                    }
				},
				"-с" => {
					config = None;
					break;
				},
				"-h" => {
					config = None;
					break;
				},
                _ => continue
            }
        }

        match config {
            Some(config) => {
                return config
            }
            None => {
                println!("Помощь:\n\
                -e path => Для кодирования большого файла\n\
                -d path => Для декодирования файла при помощи .meta файла\n\
				-n => Кол-во частей (max = 255)\n\
				-s => Размер пакета\n\
				-c => Компрессия (Not supported)\n"
                );
                std::process::exit(1);
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            method: '_',
            path: PathBuf::new(),
            options: Options::default(),
        }
    }
}

//*
//* Format packet:
//* |---hash----|-----data-----|
//* |--[u8;16]--|-----[u8]-----|
//*
//* Format MetaFile:
//* |--file_extension_len--|--file_extension--|--count_hash--|----hash_parts----|
//* |-------usize----------|-------[u8]-------|-----usize----|--Array<[u8;16]>--|
//*