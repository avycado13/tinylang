pub mod ast;
pub mod interpreter;

use std::fs;
use std::path::PathBuf;

use clap::Parser;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    tinylang
);

use interpreter::Interpreter;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "Path to .tl file")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let source = if let Some(path) = args.file {
        fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Error reading file {}: {}", path.display(), e);
            std::process::exit(1);
        })
    } else {
        String::from(r#"
puts "=== Testing File I/O ===";
write_file("test.txt", "Hello from tinylang!");
content = read_file("test.txt");
puts "File content: " + content;

puts "=== Testing elsif ===";
x = 5;
if x < 0 then
    puts "negative";
elsif x == 0 then
    puts "zero";
elsif x < 10 then
    puts "small positive";
else
    puts "large";
end;

puts "=== Testing HTTP fetch ===";
data = fetch("https://api.github.com");
puts "Fetched " + data;
"#)
    };

    let parser = tinylang::ProgramParser::new();
    let program = parser.parse(&source).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });

    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}
