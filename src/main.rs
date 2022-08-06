use clap::Parser;
use five_letters::*;

#[derive(Parser,Debug)]
#[clap()]
struct Args {
    #[clap(short,long,value_parser,default_value="words_alpha.txt")]
    filename: String,

    #[clap(short,long,value_parser)]
    intersect_with: Option<String>,

    #[clap(short,long)]
    clique_search: bool,

    #[clap(short,long)]
    keep_anagrams: bool,
}

fn main() {
    let args = Args::parse();

    let (mut words, mut bits) = load(&args.filename);
    if !args.keep_anagrams {
        (words,bits) = remove_anagrams((words,bits));
    }
    if let Some(f) = args.intersect_with {
        (words,bits) = intersect_with((words,bits), &f);
    }
    let n = words.len();
    assert!(n <= u16::MAX.into());
    
    let solutions =
        if args.clique_search {
            let neighborhoods = generate_neighborhoods(&bits);
            eprint!("\n");
            let solutions = find_cliques(neighborhoods);
            eprint!("\n");
            solutions
        } else {
            let mut dancing_links = init_dancing_links(&bits);
            eprint!("\n");
            let solutions = dancing_links.solve();
            eprint!("\n");
            solutions
        };

    println!("{}", fmt_solutions(&words, solutions));
}
