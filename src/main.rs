use five_letters::*;

fn main() {
    let (words,bits) = intersect_with(remove_anagrams(load("words_alpha.txt")),"words_wordle.txt");
    let n = words.len();
    assert!(n <= u16::MAX.into());
    /*
    let neighborhoods = generate_neighborhoods(&bits);
    eprint!("\n");
    let solutions = find_cliques(neighborhoods);
    eprint!("\n");
    */
    let mut dancing_links = init_dancing_links(&bits);
    eprint!("\n");
    let solutions = dancing_links.solve();
    eprint!("\n");
    println!("{}", fmt_solutions(&words, solutions));
}
