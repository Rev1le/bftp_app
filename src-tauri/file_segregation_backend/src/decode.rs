use std::{
    fs::{self, File},
    io::{self, Read, Write, BufReader, Error},
    path::{self, PathBuf},
    borrow::Cow
};
use std::string::FromUtf8Error;

use crate::{CompositeFile, FilePart};

struct MetaFile {

}

#[derive(Debug)]
pub enum DecodeErrors {
    IOError(::std::io::Error),
    FromUtf8Error(::std::string::FromUtf8Error),
    IterationError,
    DecodePart(usize),
	PathParseError,
}

impl From<std::io::Error> for DecodeErrors {
    fn from(value: Error) -> Self {
        DecodeErrors::IOError(value)
    }
}

impl From<std::string::FromUtf8Error> for DecodeErrors {
    fn from(value: FromUtf8Error) -> Self {
        DecodeErrors::FromUtf8Error(value)
    }
}


#[derive(Debug)]
struct HashPart([u8;16]);

impl HashPart {
    fn read(src: &mut impl Read) -> Self {

        let mut bytes = [0_u8;16];
        src.read(&mut bytes).unwrap();

        HashPart(bytes)
    }
}

pub trait DecodeType: Sized {
    fn decode_from_iter(iter: &mut impl Iterator<Item=u8>) -> Result<Self, DecodeErrors>;
}

impl DecodeType for u8 {
    fn decode_from_iter(iter: &mut impl Iterator<Item=u8>) -> Result<Self, DecodeErrors> {
        iter.next().ok_or(DecodeErrors::IterationError)
    }
}

impl DecodeType for usize {
    fn decode_from_iter(iter: &mut impl Iterator<Item=u8>) -> Result<Self, DecodeErrors> {
        let mut buffer_bytes = [0_u8;8];
        for i in 0..std::mem::size_of::<Self>() {
            buffer_bytes[i] = iter.next().ok_or(DecodeErrors::IterationError)?;
        }
        Ok(<usize>::from_be_bytes(buffer_bytes))
    }
}

fn decode_str<T: DecodeType + Into<usize>>(iter: &mut impl Iterator<Item=u8>) -> Result<String, DecodeErrors> {
    let len_str = T::decode_from_iter(iter)?.into();

    let mut output_str_bytes = Vec::with_capacity(len_str);

    for _ in 0..len_str {
        output_str_bytes.push(
            iter.next().ok_or(DecodeErrors::IterationError)?
        );
    }

    Ok(String::from_utf8(output_str_bytes)?)
}

pub fn decode_file(path: &PathBuf, path_for_save: PathBuf) -> Result<(), DecodeErrors> {

    if !path.is_file() {
        println!(
            "\n!!!!!!!!!!!!!\nПредоставьте путь к файлу сборки\n!!!!!!!!!!!!!\n"
        );
        std::process::exit(2)
    }
	
	if !path_for_save.is_dir() {
		return Err(DecodeErrors::PathParseError)
	}

    let mut parts_folder = path.clone();
    parts_folder.pop();

    let mut metafile_bytes = vec![];
    File::open(&path)?.read_to_end(&mut metafile_bytes)?;

    let mut metafile_bytes_iter = metafile_bytes.into_iter();

    let source_filename = decode_str::<u8>(&mut metafile_bytes_iter)?;
    let source_format = decode_str::<u8>(&mut metafile_bytes_iter)?;
    let parts_uuid = decode_str::<u8>(&mut metafile_bytes_iter)?;

    let mut output_file = File::create(
        format!("{}{}.{}",path_for_save.display(), source_filename, source_format)
    )?;

    // Остаточные байты представляют из себя массив хешей,
    // где кол-во хешей берется из контекста
    let count_parts = <usize>::decode_from_iter(&mut metafile_bytes_iter)?;
    let parts_hashes = metafile_bytes_iter.collect::<Vec<u8>>();

    if count_parts != parts_hashes.len()/16 {
        println!(
            "\n!!!!!!!!!!!!!\nОшибка кол-ва частей в сборочном файле.\n!!!!!!!!!!!!!\n"
        );
        std::process::exit(2)
    }

    parts_hashes
        .chunks(16)
        .enumerate()
        .map(|(part_ind, part_hash)| {
            let mut part = decode_part(
                &parts_folder,
                &parts_uuid,
                part_ind+1,
                part_hash
            );

            let mut bytes_part = vec![];
			// При большой чатси уходит много ОЗУ
            part.part_file.read_to_end(&mut bytes_part)?;

            output_file.write(&bytes_part)?;
			// Создание евента для frontend
			let _ = window.emit("decode://progress", format!("Часть: {} собрана", part_ind+1));
            Ok::<(), DecodeErrors>(())
        })
        .all(|res|
            if res.is_ok() {
                true
            } else {
                println!(
                    "\n!!!!!!!!!!!!!\nНекорректное декодирование части с ошибкой {:?}\n!!!!!!!!!!!!!\n",
                    res.err().unwrap()
                );
                std::process::exit(2)
            }
        );

    Ok(())
}

fn decode_input_file_extension(src: &mut impl Read) -> String {
    let mut extension_bytes_len = [0_u8;8];
    src.read( &mut extension_bytes_len).unwrap();
    let mut extension_len =<usize>::from_be_bytes(extension_bytes_len);

    let mut extension_bytes = Vec::with_capacity(extension_len);
    unsafe {
        extension_bytes.set_len(extension_len);
    }

    src.read(&mut extension_bytes).unwrap();

    String::from_utf8(extension_bytes).unwrap()
}

fn decode_metafile() -> MetaFile {
    MetaFile {

    }
}

fn decode_part(parts_folder: &PathBuf, part_uuid: &str, part_number: usize, part_hash: &[u8]) -> FilePart {

    let part_file_name = format!("{}_{}.part", part_uuid, part_number);
    let mut part_path = parts_folder.clone();
    part_path.push(part_file_name.clone());
    println!("ДЕЮАГ ИНФА: {:?}", &part_path);

    let mut part_file = if let Ok(f) = File::open(&part_path) {
        f
    } else {
        println!(
            "\n!!!!!!!!!!!!!\nНеудается найти {} часть в текущей папке.\n!!!!!!!!!!!!!\n",
            part_number
        );
        std::process::exit(2)
    };

    let mut tmp_hash = [0_u8;16];
    part_file.read(&mut tmp_hash).unwrap();

    let hash_bytes = tmp_hash
        .into_iter()
        .enumerate()
        // Сравнение хеша, полученного из сброчного файла и хеша в файле
        .filter_map(|(index, byte)| if byte == part_hash[index] {Some(byte)} else {None} )
        .collect::<Vec<_>>();

    if hash_bytes.len() != 16 {
        println!(
            "\n!!!!!!!!!!!!!\nХеш {} части не совпадает.\n!!!!!!!!!!!!!\n",
            part_number
        );
        std::process::exit(2)
    }

    FilePart {
        part_file,
        hash_bytes,
        part_file_name,
    }
}