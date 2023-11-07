use clap::Parser;

/// Stub CLI argument parsing using clap.
// ^^ This will be used as program description - note triple slash.
// hash + square bracket is an attribute, here used as shorthand for defining behaviour of 
// the objects they're attached to
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// struct is a collection of field names with type definitions. Values are not
// assigned yet.
struct Args {

    /// Location of target tiles PDF.
    #[arg(short, long)]
    target_pdf: String,

    /// Zoomrange to slice tiles for.
    #[arg(short, long, required(false), num_args=1.., value_delimiter = ' ')]
    zoomrange: Vec<u8>,

    /// Location to write output tiles to.
    #[arg(short, long, required(false), default_value("output"))]
    output_loc: String,

}


fn main() {
    let args = Args::parse();

    println!("{}", args.target_pdf);
    println!("{:?}", args.zoomrange);
    println!("{}", args.output_loc);
}
