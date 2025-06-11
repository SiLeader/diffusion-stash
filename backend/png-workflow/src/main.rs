use clap::Parser;
use png_workflow::GenerationInfo;

#[derive(Debug, Parser)]
struct Args {
    file_path: String,
}

fn main() {
    let args = Args::parse();
    match GenerationInfo::from_path(args.file_path) {
        Ok(info) => {
            print_info(&info);
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
}

fn print_info(info: &GenerationInfo) {
    fn format_option<T: ToString>(opt: &Option<T>) -> String {
        opt.as_ref().map_or("N/A".to_string(), |v| v.to_string())
    }

    println!("-- Model file name:\n{}", info.model_names.join("\n"));
    println!(
        "-- Positive prompt:\n{}",
        format_option(&info.positive_prompt)
    );
    println!(
        "-- Negative prompt:\n{}",
        format_option(&info.negative_prompt)
    );
    println!("-- Steps: {}", format_option(&info.step_count));
    println!("-- CFG: {}", format_option(&info.cfg_scale));
    println!("-- Seed: {}", format_option(&info.seed));
    println!("-- Sampler: {}", format_option(&info.sampler_name));
    println!("-- Scheduler: {}", format_option(&info.scheduler_name));
}
