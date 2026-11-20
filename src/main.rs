use std::fs;
use std::fs::{Metadata, File};
use std::io::{BufWriter, Read, Write, stdout};
use std::env::args;
use std::path;
use std::path::PathBuf;
use std::fs::read_dir;

const KB: u64 = 1024;
const MB: u64 = KB * 1024;
const GB: u64 = MB * 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get arguments
    let args = get_args();

    let source = path::Path::new(&args[1]).canonicalize()?; // Source path
    let dest = path::Path::new(&args[2]).canonicalize()?; // Destination path

    let mut buffer = [0u8; 65536]; // Buffer

    // Convert the source name to string
    let source_name_string = source.to_string_lossy().to_string();

    let source_metadata = source.metadata()?; // Source metadata

    // Convert the destination to string
    let dest_dir = dest.to_string_lossy().to_string();

    let mut count: u64 = 0; // Counter

    match args.len() {
        3 => {
            if source.is_file() {
                let mut source_file = File::open(&source)?;

                if !dest.is_dir() {
                    eprintln!("{} is not a directory!\nDestination should be a directory!", dest_dir);
                    std::process::exit(1);
                }

                let dest_file = source.file_name().unwrap().to_string_lossy().to_string(); // Destination file string using source file name

                let dest_path = dest.join(dest_file); // Destination file path

                let file_copy = File::create_new(dest_path)?; // Creating the destination file

                let mut byte_writer = BufWriter::new(file_copy);

                println!("Copying: {}", source_name_string);

                loop {
                    let bytes = source_file.read(&mut buffer)?;

                    if bytes == 0 {
                        break;
                    }

                    byte_writer.write_all(&buffer[..bytes])?;

                    count += bytes as u64;

                    print_progress(count, &source_metadata);
                }

                byte_writer.flush()?;

                println!();

                println!("Successfully copied: {} to {}", source_name_string, dest_dir);
            } else if source.is_dir() {
                let root_destination = dest.join(source.file_name().unwrap());
                fs::create_dir_all(&root_destination)?;

                let dir_contents = recursive_folder(&source);

                for contents in dir_contents {
                    let source_metadata = contents.metadata()?;

                    if contents.is_dir() {
                        let stripped = contents.strip_prefix(&source)?;

                        let destination = root_destination.join(stripped);

                        fs::create_dir_all(destination)?;
                        
                    } else if contents.is_file() {
                        let mut count: u64 = 0;

                        let mut source_file = File::open(&contents)?;

                        if !dest.is_dir() {
                            eprintln!("{} is not a directory!\nDestination should be a directory!", dest_dir);
                            std::process::exit(1);
                        }

                        let strip_filename = contents.strip_prefix(&source)?; // Strip file name to get directory below source

                        let dest_join = dest.join(source.file_name().unwrap()).join(strip_filename); // Destination file

                        let file_copy = File::create_new(dest_join)?; // Creating the destination file (THIS IS WHERE IT PANICS)

                        let mut byte_writer = BufWriter::new(file_copy);

                        println!("Copying: {}", contents.to_string_lossy());

                        loop {
                            let bytes = source_file.read(&mut buffer)?;

                            if bytes == 0 {
                                break;
                            }

                            byte_writer.write_all(&buffer[..bytes])?;

                            count += bytes as u64;

                            print_progress(count, &source_metadata);
                        }

                        byte_writer.flush()?;

                        println!();

                        println!("Successfully copied: {} to {}\n", contents.to_string_lossy(), dest_dir);
                    }
                }
            }
        },

        5 => {
            let cp_file_name = &args[3]; // name argument '--name'

            if source.is_file() {
                let mut source_file = File::open(&source)?;

                if !dest.is_dir() {
                    eprintln!("{} is not a directory!\nDestination should be a directory!", dest_dir);
                    std::process::exit(1);
                }

                if cp_file_name == "--name" {
                    let dest_file_name = &args[4]; // Destination file name

                    let dest_path = dest.join(dest_file_name); // Destination file path

                    let dest_name_path = dest_path.to_string_lossy().to_string();

                    let file_copy = File::create_new(dest_path)?; // Creating the destination file

                    let mut byte_writer = BufWriter::new(file_copy);

                    println!("Copying: {}", source_name_string);

                    loop {
                        let bytes = source_file.read(&mut buffer)?;

                        if bytes == 0 {
                            break;
                        }

                        byte_writer.write_all(&buffer[..bytes])?;

                        count += bytes as u64;

                        print_progress(count, &source_metadata);
                    }

                    byte_writer.flush()?;

                    println!();

                    println!("Successfully copied: {} to {}", source_name_string, dest_name_path);

                } else {
                    eprintln!("For custom file name use the argument: --name");
                    std::process::exit(1);
                }
            } else if source.is_dir() {
                if cp_file_name == "--name" {
                    let dest_file_name = &args[4]; // Destination file name

                    let root_destination = dest.join(dest_file_name);
                    fs::create_dir_all(&root_destination)?;

                    let dir_contents = recursive_folder(&source);

                    for contents in dir_contents {
                        let source_metadata = contents.metadata()?;

                        if contents.is_dir() {
                            let stripped = contents.strip_prefix(&source)?;

                            let destination = root_destination.join(stripped);

                            fs::create_dir_all(&destination)?;
                            
                        } else if contents.is_file() {
                            let mut count: u64 = 0;

                            let mut source_file = File::open(&contents)?;

                            if !dest.is_dir() {
                                eprintln!("{} is not a directory!\nDestination should be a directory!", dest_dir);
                                std::process::exit(1);
                            }

                            let strip_filename = contents.strip_prefix(&source)?; // Strip file name to get directory below source

                            let dest_join = root_destination.join(strip_filename); // Destination file

                            let file_copy = File::create_new(dest_join)?; // Creating the destination file (THIS IS WHERE IT PANICS)

                            let mut byte_writer = BufWriter::new(file_copy);

                            println!("Copying: {}", contents.to_string_lossy());

                            loop {
                                let bytes = source_file.read(&mut buffer)?;

                                if bytes == 0 {
                                    break;
                                }

                                byte_writer.write_all(&buffer[..bytes])?;

                                count += bytes as u64;

                                print_progress(count, &source_metadata);
                            }

                            byte_writer.flush()?;

                            println!();

                            println!("Successfully copied: {} to {}\n", contents.to_string_lossy(), dest_dir);
                        }
                    }
                
                } else {
                    eprintln!("If you want to use a custom file name, use the argument: --name");
                    std::process::exit(1);
                }
            }
        },

        _ => {
            println!("Usage: dkcp <source> <destination_dir>");
            println!("Usage: dkcp <source> <destination_dir> --name <desired file name.ext>");
            std::process::exit(1)
        }
    }

    Ok(())
}

fn get_args() -> Vec<String> {
    let arguments: Vec<String> = args().collect();

    match arguments.len() {
        3 => {
            arguments
        }

        5 => {
            arguments
        }

        _ => {
            println!("Usage:");
            println!("\tdkcp <source> <destination_dir>");
            println!("\tdkcp <source> <destination_dir> --name <desired_file_name.ext>");
            println!("\tdkcp <source> <destination_dir> --name <desired_dir_name>");
            std::process::exit(1)
        }
    }
}

fn recursive_folder(source_dir: &path::Path) -> Vec<PathBuf> {
    let mut files_vec = Vec::new();

    if let Ok(entries) = read_dir(source_dir) {
        for entry in entries {
            let entry = entry.expect("Failed to read entry!");
            let path = entry.path();

            let path_str = path.to_string_lossy().to_string(); // Create a string of the path

            let mut new_path = PathBuf::new(); // Create new pathbuf to push the path string so as to not have ownership issues with path

            new_path.push(path_str); // Push path_string to new_path

            if path.is_dir() {
                files_vec.push(new_path);

                let mut sub_dir = recursive_folder(&path);

                files_vec.append(&mut sub_dir);
            } else if path.is_file() {

                files_vec.push(path);
            }
        }
    }

    files_vec
}

fn print_progress(count: u64, source_metadata: &Metadata) {
    if source_metadata.len() >= GB {
        let percentage = count as f64 / (source_metadata.len() as f64) * 100.0;
        print!("\r\x1b[KProgress: {:.2}%\t [{:.2} GiB/{:.2} GiB]", percentage, (count as f64 / GB as f64), (source_metadata.len() as f64 / GB as f64));
        stdout().flush().unwrap();

    } else if source_metadata.len() >= MB {
        let percentage = count as f64 / (source_metadata.len() as f64) * 100.0;
        print!("\r\x1b[KProgress: {:.2}%\t [{:.2} MiB/{:.2} MiB]", percentage, (count as f64 / MB as f64), (source_metadata.len() as f64 / MB as f64));
        stdout().flush().unwrap();

    } else if source_metadata.len() >= KB {
        let percentage = count as f64 / (source_metadata.len() as f64) * 100.0;
        print!("\r\x1b[KProgress: {:.2}%\t [{:.2} KiB/{:.2} KiB]", percentage, (count as f64 / KB as f64), (source_metadata.len() as f64 / KB as f64));
        stdout().flush().unwrap();

    } else {
        let percentage = count as f64 / (source_metadata.len() as f64) * 100.0;
        print!("\r\x1b[KProgress: {:.2}%\t [{:.2} Bytes/{:.2} Bytes]", percentage, count as f64, source_metadata.len() as f64);
        stdout().flush().unwrap();
    }
}
