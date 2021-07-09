use colored::Colorize;
use regex_gather;

fn main() {
    let input = [
        "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4",
        "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 02 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4",
    ];
    println!("{}", "# gather from at least 2".red());
    println!("input:\n  {:?}", &input);
    println!(
        "out:\n  {}",
        regex_gather::gather_regex_from_at_least_2(&input, "(?P<ep>\\d+)", true).unwrap()
    );
    println!("");

    let input = [
        "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4",
        "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 02 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4",
    ];
    println!("{}", "# gather from at least 2".red());
    println!("input:\n  {:?}", &input);
    println!(
        "out:\n  {}",
        regex_gather::gather_pattern(&input, "{ep}", true).unwrap()
    );
    println!("");

    let input = "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4";
    println!("{}", "# guess from one".red());
    println!("input:\n  {}", &input);
    let outs = regex_gather::guess_pattern_with_escape(input, "(?P<ep>\\d+)", regex::escape).unwrap();
    println!("out:");
    for out in outs {
        println!(
            " - {}\n - {}{}{}\n",
            &out.pattern,
            &out.preview_left,
            &out.highlight.bold().green(),
            &out.preview_right
        );
    }

    let input = "[Lilith-Raws] 桃子男孩渡海而来 / Peach Boy Riverside - 01 [Baha][WEB-DL][1080p][AVC AAC][CHT].mp4";
    println!("{}", "# guess template".red());
    println!("input:\n  {}", &input);
    let outs = regex_gather::guess_pattern_with_escape(input, "{ep}", |s| s.to_owned()).unwrap();
    println!("out:");
    for out in outs {
        println!(
            " - {}\n - {}{}{}\n",
            &out.pattern,
            &out.preview_left,
            &out.highlight.bold().green(),
            &out.preview_right
        );
    }
}
