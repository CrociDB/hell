use std::env;
use std::fs;

use crate::defs;

#[derive(Debug)]
struct ExecFile {
    filename: std::ffi::OsString,
    filepath: std::path::PathBuf,
}

fn get_exec_files(comm: &str) -> Result<Vec<ExecFile>, defs::CheckerError> {
    let mut vec: Vec<ExecFile> = Vec::new();

    if let Ok(path) = env::var("PATH") {
        let paths: Vec<&str> = path.trim().split(':').collect();

        // gets all the paths
        for p in paths {
            let file_in_path_res = fs::read_dir(p);
            if file_in_path_res.is_ok() {
                let file_in_path: Result<Vec<fs::DirEntry>, std::io::Error> = file_in_path_res
                    .unwrap()
                    .filter_map(|entry| {
                        let entryu = entry.as_ref().unwrap();
                        if entryu.file_name() == comm {
                            if let Ok(metadata) = entryu.metadata() {
                                if metadata.is_file() {
                                    return Some(entry);
                                }
                            }
                            None
                        } else {
                            None
                        }
                    })
                    .collect();

                let files = file_in_path.unwrap();
                if !files.is_empty() {
                    let f = &files[0];
                    vec.push(ExecFile {
                        filename: f.file_name(),
                        filepath: f.path(),
                    });
                }
            }
        }
    } else {
        eprintln!("Error: no PATH environment variable found");
        return Err(defs::CheckerError::Other("no PATH"));
    }

    Ok(vec)
}

pub fn check_exec(line: &Vec<&str>) -> Result<(), defs::CheckerError> {
    let comm = line[0].trim();

    let exec_files = get_exec_files(comm)?;
    if exec_files.len() > 0 {
        println!(
            "Running {}: {}",
            exec_files[0].filename.display(),
            exec_files[0].filepath.display()
        );
        return Ok(());
    }

    for e in exec_files {
        println!("{}", e.filepath.display());
    }

    Err(defs::CheckerError::NotFound)
}

pub fn check_type_exec(comm: &str) -> bool {
    match get_exec_files(comm) {
        Ok(f) => {
            if !f.is_empty() {
                println!("{} is {}", comm, f[0].filepath.display());
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
