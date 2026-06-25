use std::fs;
use clap::Parser;
use std::io::{self, Write};
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    fname: String, 

    #[arg(long)]
    width: Option<usize>,

    #[arg(long)]
    limit: Option<usize>,
}

struct Chunk<'a> {
    data: &'a [u8]
}

impl Chunk<'_> {
    fn to_hex(&self) -> String {
        let hex: String = self.data
            .iter()
            .map(|byte| format!("{:02x} ", byte))
            .collect();
        hex
    }       

    fn to_ascii(&self) -> String {
        let ascii: String = self.data
            .iter()
            .map(|&byte| {
                if byte.is_ascii_graphic() || byte == b' ' {
                    byte as char
                } else {
                    '.'
                }
            })
            .collect();
        ascii
    }
}

fn get_limit( arg_limit: Option<usize>, max_limit: usize ) -> usize {
    let limit: usize;
    match arg_limit {
        Some(val) => limit = if val < max_limit { val } else { max_limit },
        None => limit = max_limit,
    }
    limit
    arg_limit.unwrap_or(max_limit).min(max_limit)
}

fn get_width( arg_width: Option<usize> ) -> usize {
    let buffer_size: usize;
    match arg_width {
        Some(val) => buffer_size = val,
        None => buffer_size = 16,
    }
    return buffer_size
    arg_width.unwrap_or(16)
} 

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    eprintln!("Binary file: {}", args.fname);
    let data: Vec<u8> = fs::read(args.fname)?;
    let buffer_size: usize = get_width(args.width);
    let limit: usize = get_limit(args.limit, data.len());

    if buffer_size == 0 {
        eprintln!("width must be greater than 0");
        process::exit(1);
    }

    //  This is needed to allow piping to more or less
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for (i, chunk) in data.chunks(buffer_size).enumerate() {
        let offset = i * buffer_size;

        let buffer = Chunk{data: {chunk}};

        if (i + 1) * buffer_size > limit {
            buffer.data = &buffer.data[..(limit-offset)];
        }

        if let Err(e) = writeln!( handle, "{:08x}:   {}| {} |", offset, buffer.to_hex(), buffer.to_ascii()) {
            if e.kind() == io::ErrorKind::BrokenPipe {
                // The user quit 'more' or 'less' early. Exit cleanly.
                process::exit(0);
            }
            // If it's a different error, log it to stderr and exit with an error code
            eprintln!("Error writing to stdout: {}", e);
            process::exit(1);
        }

        if (i+1)*buffer_size > limit { break };

    }

    Ok(())

}
