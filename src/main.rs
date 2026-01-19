use clap::Parser;
use anyhow::{Context, Result};
use log::{info, warn};

#[derive(Parser)]
struct Cli {

    pattern: String,
    path: std::path::PathBuf,
}

fn main()-> Result<()> {
    env_logger::init();
    let args = Cli::parse();
    
    // 1.5. log
    info!("starting up"); //RUST_LOG=info cargo run -- d test.txt
    warn!("oops, nothing implemented!");
    
    //1.2 = File path and pattern to search for
    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
    
    //1.3 (grep command) = It's taking a stream of data, filtering it, and only outputting what you want. try->cargo run -- d test.txt
    
  /*  let content = std::fs::read_to_string(&args.path).expect("could not read file!");
    for line in content.lines() {
        
        if line.contains(&args.pattern) {
           
           println!("{}", line);
        }
    }*/
    
    //1.4 nicer error reporting try -> cargo run -- test yok.txt, cargo run -- d test.txt
    
   let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

 /*   for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    } */
    
    //1.6 testing
    find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    
    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write)-> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
           let _ =writeln!(writer, "{}", line);
        }
    }
     Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_a_match() {
        let mut result = Vec::new();

        find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).unwrap();
        
        assert_eq!(result, b"lorem ipsum\n");
    }
}
