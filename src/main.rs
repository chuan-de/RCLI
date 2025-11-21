use RCLI::{
    Base64SubCommand, Opts, Subcommand, TextSignFormat, TextSubCommand, process_csv,
    process_decode, process_encode, process_genpass, process_text_sign, process_text_verify,
};
use clap::Parser;

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
        Subcommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        Subcommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
        Subcommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    process_text_sign(&opts.input, &opts.key, opts.format)?;
                }
                TextSignFormat::Ed25519 => {
                    process_text_sign(&opts.input, &opts.key, opts.format)?;
                }
            },
            TextSubCommand::Verify(opts) => match opts.format {
                TextSignFormat::Blake3 => {
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                }
                TextSignFormat::Ed25519 => {
                    process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                }
            },
        },
    }
    Ok(())
}
