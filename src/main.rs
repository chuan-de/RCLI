use RCLI::{
    Base64SubCommand, Opts, Subcommand, TextSignFormat, TextSubCommand, process_csv,
    process_decode, process_encode, process_generate, process_genpass, process_text_sign,
    process_text_verify,
};
use clap::Parser;
use std::fs;
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?
        }
        Subcommand::GenPass(opts) => {
            let generated_password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", generated_password);
            let estimate = zxcvbn(&generated_password, &[]);
            eprintln!("Password strength estimate: {}", estimate.score());
        }
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}", encode);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", decoded);
            }
        },
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                    println!("{}", sig);
                }
                TextSignFormat::Ed25519 => {
                    let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                    println!("{}", sig);
                }
            },
            TextSubCommand::Verify(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    let verified =
                        process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                    println!("{}", verified);
                }
                TextSignFormat::Ed25519 => {
                    let verified =
                        process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                    println!("{}", verified);
                }
            },
            TextSubCommand::Generate(opts) => {
                let keys = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &keys[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &keys[0])?;
                        fs::write(name.join("ed25519.pk"), &keys[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
