use five_letters::*;

fn main() {
    let (words,bits) = intersect_with(remove_anagrams(load("words_alpha.txt")),"words_wordle.txt");
    let n = words.len();
    assert!(n <= u16::MAX.into());
    /*
    let neighborhoods = generate_neighborhoods(&bits);
    let solutions = find_cliques(neighborhoods);
    */
    let mut dancing_links = init_dancing_links(&bits);
    let solutions = dancing_links.solve();
    println!("{}", fmt_solutions(&words, solutions));
}
