use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File, metadata};
use std::io::{self, BufReader, BufWriter, copy, Result};
use std::path::Path;

fn compress_file(source_path: &str, target_path: &str) -> Result<()> {
    let start = std::time::Instant::now();

    let source_file = File::open(source_path)?;
    let source_metadata = metadata(source_path)?;

    let mut input = BufReader::new(source_file);
    let output = BufWriter::new(File::create(target_path)?);
    let mut encoder = GzEncoder::new(output, Compression::default());

    copy(&mut input, &mut encoder)?;

    encoder.finish()?;

    let target_metadata = metadata(target_path)?;

    println!("Source len: {:?}", source_metadata.len());
    println!("Target len: {:?}", target_metadata.len());
    println!("Elapsed: {:?}", start.elapsed());

    Ok(())
}

fn decompress_file(zip_file: &Path) -> io::Result<()> {
    let file = File::open(zip_file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
        }

        if file.is_dir() {
            println!("Extracting directory: {}", outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!(
                "Extracting file: {} ({} bytes)",
                outpath.display(),
                file.size()
            );
            let mut outfile = BufWriter::new(File::create(&outpath)?);
            io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Welcome to Compression and Decompression program!");

    loop {
        println!("Enter 'c' to compress or 'd' to decompress (q to quit):");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "c" => {
                println!("Enter source file path:");
                let mut source_path = String::new();
                io::stdin().read_line(&mut source_path).expect("Failed to read line");
                let source_path = source_path.trim();

                println!("Enter target file path:");
                let mut target_path = String::new();
                io::stdin().read_line(&mut target_path).expect("Failed to read line");
                let target_path = target_path.trim();

                compress_file(&source_path, &target_path)?;
                println!("File compressed successfully!");
            }
            "d" => {
                println!("Enter zip file path:");
                let mut zip_file_path = String::new();
                io::stdin().read_line(&mut zip_file_path).expect("Failed to read line");
                let zip_file_path = zip_file_path.trim();

                let zip_file = Path::new(&zip_file_path);
                decompress_file(&zip_file)?;
                println!("File decompressed successfully!");
            }
            "q" => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please enter 'c', 'd', or 'q'."),
        }
    }

    Ok(())
}
