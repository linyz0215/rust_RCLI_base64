use clap::Parser;

use RCLI::{Opts, SubCommand, process_csv, process_genpass};
use anyhow::Result;

fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {//output类型为Option<String> Some or None
                output.clone()
            } else {
                format!("output.{}",opts.format.as_str())//直接实现OutputFormat的as_str方法
            };
            process_csv(&opts.input, output,opts.format)?;
        }

        SubCommand::GenPass(opts) => {
            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.numbers, opts.symbols)?;
        }
    }
    Ok(())
}
