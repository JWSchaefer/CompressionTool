use compression::huffman::Huffman;
use console::style;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::fs;
use unbytify::bytify;

fn main() {
    let source_path = "examples/resources";
    let entries = fs::read_dir(source_path).expect("Failed to read directory");

    let n_paths = fs::read_dir(source_path)
        .expect("Failed to read directory")
        .count() as u64;
    let mut errors = Vec::new();

    // Define progress bar style
    let bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:45.blue/dim}] {pos:>3}/{len} ({eta:>5})")
        .expect("Invalid progress bar style template")
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:2>.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("-".repeat(3).as_str());

    let pb = ProgressBar::new(n_paths);
    pb.set_style(bar_style);
    pb.inc(1);

    for entry in entries {
        let pathbuf = entry.expect("Failed to get path").path();
        let file_name = pathbuf
            .file_stem()
            .expect("Failed to get file stem")
            .to_str()
            .expect("Failed to convert file stem to string")
            .trim()
            .to_string();

        // Read file contents
        let source = match fs::read_to_string(&pathbuf) {
            Ok(content) => content,
            Err(_) => {
                pb.println(format!(
                    "❌ {:>9}: {}",
                    style("Failed").red(),
                    file_name
                ));
                errors.push(file_name);
                continue;
            }
        };

        // Encode and decode the content
        let encoding = Huffman::encode(source.clone())
            .expect("Failed to encode the source file.");

        let result = Huffman::decode(encoding.clone())
            .expect("Failed to decode the encoded source file.");

        if source != result {
            pb.println(format!("❌ Failed: {}", file_name));
            errors.push(file_name);
            continue;
        }

        // Compute and display size ratio
        let encoding_bytes_value = encoding.len();
        let source_bytes_value = source.as_bytes().len();
        let size_ratio =
            100.0 * encoding_bytes_value as f64 / source_bytes_value as f64;

        let (encoding_bytes_value, encoding_bytes_suffix) =
            bytify(encoding_bytes_value as u64);
        let (source_bytes_value, source_bytes_suffix) =
            bytify(source_bytes_value as u64);
        let display_file_name = truncate_file_name(&file_name, 32);
        let styled_display_size_ratio = style_size_ratio(size_ratio);

        let result_string = format!(
            "📚 {}: {:35} {source_bytes_value:>3.0} {source_bytes_suffix} -> {encoding_bytes_value:>3.0} {encoding_bytes_suffix} ({})",
            style("Succeeded").green(),
            display_file_name,
            styled_display_size_ratio
        );

        pb.println(result_string);
        pb.inc(1);
    }

    // Final results
    if errors.len() == 1 && errors[0] == "This Should Fail" {
        println!("\n\n🎉 Checks completed with no erros.");
    } else {
        println!("\n\n⚠️ Checks completed with errors.");
    }
}

// Helper function to truncate file names
fn truncate_file_name(file_name: &str, max_width: usize) -> String {
    if file_name.len() > max_width {
        format!("{}...", &file_name[..max_width])
    } else {
        file_name.to_string()
    }
}

// Helper function to style size ratio
fn style_size_ratio(size_ratio: f64) -> String {
    let display_size_ratio = format!("{size_ratio:>2.1}%");
    match size_ratio {
        x if x < 60.0 => style(display_size_ratio).green().to_string(),
        x if x < 70.0 => style(display_size_ratio).yellow().to_string(),
        _ => style(display_size_ratio).red().to_string(),
    }
}
