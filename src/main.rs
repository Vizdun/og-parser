use comfy_table::Table;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    url: String,
}

fn main() {
    use webpage::{Webpage, WebpageOptions};

    let cli = Args::parse();

    let info =
        Webpage::from_url(&cli.url, WebpageOptions::default()).expect("Could not read from URL");

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::UTF8_FULL);
    table.set_header(vec!["Tag", "Value"]);

    let mut filtered = info
        .html
        .meta
        .iter()
        .filter(|(tag, _)| tag[0..2] == *"og")
        .map(|(tag, val)| (tag, val))
        .collect::<Vec<(&String, &String)>>();

    filtered.sort_by(|a, b| a.0.cmp(b.0));

    for (tag, val) in &filtered {
        table.add_row(vec![tag, val]);
    }

    println!("{}", table);
}
