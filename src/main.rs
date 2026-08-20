use clap::Parser;
use leptess::LepTess;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: String,
}

#[derive(Debug)]
struct Word {
    text: String,
    left: i32,
    top: i32,
    conf: f32,
}

fn main() {
    let cli = Cli::parse();
    let mut lt = LepTess::new(None, "eng").unwrap();
    lt.set_image(&cli.path).unwrap();
    lt.set_variable(leptess::Variable::PreserveInterwordSpaces, "11")
        .unwrap();
    let tsv = lt.get_tsv_text(0).unwrap();

    let mut words = Vec::new();

    for line in tsv.lines().skip(1) {
        let columns: Vec<&str> = line.split('\t').collect();
        // print!("{}",columns[1]);

        if columns.len() < 12 {
            continue;
        }

        let conf: f32 = columns[10].parse().unwrap_or(0.0);
        // remove garbage
        if conf < 70.0 {
            continue;
        }
        let text = columns[11].to_string();

        if text.trim().is_empty() {
            continue;
        }

        words.push(Word {
            text,
            left: columns[6].parse().unwrap(),
            top: columns[7].parse().unwrap(),
            conf,
        });
    }

    words.sort_by_key(|w| w.top);

    let mut lines: Vec<Vec<&Word>> = Vec::new();

    for word in &words {
        if let Some(last_line) = lines.last_mut() {
            let last_top = last_line[0].top;

            // same line if vertical distance is small
            if (word.top - last_top).abs() < 10 {
                last_line.push(word);
                continue;
            }
        }
        lines.push(vec![word]);
    }

    for line in &mut lines {
        line.sort_by_key(|w| w.left);
    }

    for line in lines {
        let text = line
            .iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", text);
    }
}
