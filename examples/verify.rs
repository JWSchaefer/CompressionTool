use indicatif::{ProgressState, ProgressStyle, ProgressBar};
use std::fs;
use std::fmt::Write;
use unbytify::bytify;

use compression::huffman::Huffman;

fn main() {
    let source_path = "examples/resources";
    let paths = fs::read_dir(source_path).unwrap();

    let mut errors: Vec<String> = Vec::new();

    let n_paths = fs::read_dir(source_path)
        .unwrap()
        .count() as u64;
  
    let bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");
            
    let pb = ProgressBar::new(n_paths);
    pb.set_style(bar_style);

    for path in paths {
        let pathbuf = path.unwrap().path();
        let file_name = pathbuf.file_stem().unwrap().to_str().unwrap().to_string();

        let source = match fs::read_to_string(&pathbuf) {
            Ok(source) => source,
            Err(_) => {
                pb.println(format!("❌  {}", &file_name));
                errors.push(file_name);
                continue;
            }
        };
            
        let encoding = Huffman::encode(source.clone())
            .expect("Failed to encode the source file.");

        let result = Huffman::decode(encoding.clone())
            .expect("Failed to decode the encoded source file.");
        let error = source != result;
        
        if error {
            pb.println(format!("❌ Failed: {}", &file_name));
            errors.push(file_name);
        } else {

            let encoding_bytes_value = encoding.len();
            let source_bytes_value = source.as_bytes().len();
            let size_ratio = 100. * encoding_bytes_value as f64 / source_bytes_value as f64;
            
            let (encoding_bytes_value, encoding_bytes_suffix) = bytify(encoding_bytes_value as u64);
            let (source_bytes_value, source_bytes_suffix) = bytify(source_bytes_value as u64);
            let mut display_file_name = file_name.to_string();

            const MAX_FILE_NAME_WIDTH : usize = 50;
            if MAX_FILE_NAME_WIDTH <= display_file_name.len() {
                display_file_name = display_file_name[..MAX_FILE_NAME_WIDTH].to_owned();
                display_file_name.push_str("...");
            };

            let _string = format!(
                "📚 Completed: {:64} {source_bytes_value:>5.1} {source_bytes_suffix} -> {encoding_bytes_value:>5.1} {encoding_bytes_suffix} ({size_ratio:.2}%)",
                &display_file_name.trim()
            );

            pb.println(
                format!("{}", _string)
            );
 
        }

        pb.inc(1);

        
    }

    if errors.is_empty() {
        println!("\n\n🎉 Completed checks with no issues.");
    } else {
        println!("Completed checks with errors.");
    };

    if !errors.is_empty() {
        println!("\n\nErrors found in the following texts:");
        for error in errors {
            println!("{}", error);
        }
    }
}