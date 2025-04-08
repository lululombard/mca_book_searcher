use std::sync::Arc;
use std::thread;
use std::{fs::File, sync::Mutex, env};
use std::io::{BufReader, Cursor, Read, Write};
use fastanvil::{RegionBuffer};
use fastnbt::{Value, de::from_bytes};
use rayon::scope;
use walkdir::WalkDir;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    // Get the save path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path_to_save_directory>", args[0]);
        println!("Example: {} /path/to/minecraft/world/DIM1/region", args[0]);
        return;
    }

    let save_path = &args[1];
    let region_path = Path::new(save_path);
    if !region_path.exists() || !region_path.is_dir() {
        println!("Error: The provided path '{}' doesn't exist or is not a directory", save_path);
        return;
    }

    let file_out = Arc::new(Mutex::new(BufWriter::new(File::create(r"out.txt").unwrap())));
    println!("Writing detailed output to out.txt");
    let names = ["minecraft:written_book", "minecraft:writable_book"];
    println!("Searching for {} and {}", names[0], names[1]);
    println!("Searching in path: {}", save_path);
    let start = std::time::Instant::now();
    scope(|scope_files| {
        for entry in WalkDir::new(region_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
                continue;
            }

            let file_out = file_out.clone();
            scope_files.spawn(move |_| {
                scope(|scope| {
                    let mut file = BufReader::new(File::open(entry.path()).unwrap());
                    let mut data: Vec<u8> = Vec::new();
                    file.read_to_end(&mut data).unwrap();
                    let mut region = RegionBuffer::new(Cursor::new(data));
                    region.for_each_chunk(|x,y,data|{
                        let chunk: Value = from_bytes(data.as_slice()).unwrap();
                        let file_out = file_out.clone();
                        let names = names.clone();
                        scope.spawn(move |_| {
                            for name in &names {
                                if let Some(v) = find_block(&chunk, name) {
                                    let mut lock = file_out.lock().unwrap();
                                    if let Some((x,y,z)) = coordinates(v) {
                                        writeln!(&mut lock, "Found {} at x {}, y {}, z {}: {:?}", name, x, y, z, v).unwrap();
                                        println!("Found {} at x {}, y {}, z {}", name, x, y, z);
                                    } else {
                                        writeln!(&mut lock, "Found {}: {:?}", name, v).unwrap();
                                        println!("Found {}", name);
                                    }
                                }
                            }
                        });
                    }).unwrap();
                    println!("{}", entry.path().display());
                });
            });
        }
    });
    let end = start.elapsed();
    println!("Took {}ms",end.as_millis());

    let mut lock = file_out.lock().unwrap();
    lock.flush().unwrap();
    println!("finished");
}

fn coordinates(value: &Value) -> Option<(i32,i32,i32)> {
    if let Value::Compound(map) = value {
        if let (Some(Value::Int(x)), Some(Value::Int(y)), Some(Value::Int(z))) = (map.get("x"), map.get("y"), map.get("z")) {
            return Some((*x,*y,*z));
        }
    }
    None
}

fn is_primitive(value: &Value) -> bool {
    match value {
        Value::Compound(_) => false,
        Value::List(_) => false,
        _ => true
    }
}

fn find_block<'a>(value: &'a Value, name: &str) -> Option<&'a Value> {
    match value {
        Value::String(s) => match s.contains(name) {
            true => Some(value),
            false => None,
        },
        Value::List(v) => {
            for val in v {
                // don't report the primitive
                match find_block(val, name) {
                    Some(v) => {
                        if is_primitive(v) {
                            return Some(value);
                        } else {
                            return Some(v);
                        }
                    }
                    None => (),
                }
            }
            None
        },
        Value::Compound(map) => {
            for (key,val) in map.iter() {
                if key.contains(name) {
                    return Some(value)
                }
                // don't report the primitive
                match find_block(val, name) {
                    Some(v) => {
                        if is_primitive(v) {
                            return Some(value);
                        } else {
                            return Some(v);
                        }
                    }
                    None => (),
                }
            }
            None
        },
        _ => None
    }
}