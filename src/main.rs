use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::io;
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

fn main() {
    let lines_guesses = lines_from_file("wordle-nyt-allowed-guesses.txt");
    let lines_alphabe = lines_from_file("wordle-nyt-answers-alphabetical.txt");
    let words: Vec<String> = lines_guesses.into_iter().chain(lines_alphabe.into_iter()).collect();

    //println!("Lines in words {}", words.len());

    let words_five = words.into_iter()
        .filter(|word| word.len() == 5)
        .collect::<Vec<_>>();

    println!("Words with 5 letters {}", words_five.len());
    //vec_head(&words_five);

    let mut enc_words: Vec<u32> = words_five
        .iter()
        .map(|x| encode_word(x))
        .filter(|e| e.count_ones()==5)
        .collect();

    enc_words.sort();
    enc_words.dedup();

    //enc_head(&enc_words);
    let enc_len = enc_words.len();

    println!("Cooked words {}", enc_len);

    let i_arr: Vec<usize>  = (0..enc_len).collect();
    i_arr
        .par_iter()
        .progress_count(i_arr.len() as u64)
        .for_each(|i| search_second(*i, &enc_words, &words_five));
    println!("End par iter");
}

fn search_second(i: usize, enc_words: &Vec<u32>, words_five: &Vec<String>){
    let enc_len: usize  = enc_words.len().try_into().unwrap();

    let a = enc_words[i];

    for j in (i+1)..enc_len{
        let b = enc_words[j];
        if (a & b) != 0 { continue; }
        let ab = a | b;

        for k in (j+1)..enc_len{
            let c = enc_words[k];
            if (ab & c) != 0 { continue; }
            let abc = ab | c;

            for l in (k+1)..enc_len{
                let d = enc_words[l];
                if (abc & d) != 0 { continue; }
                let abcd = abc | d;

                for m in (l+1)..enc_len{
                    let e = enc_words[m];
                    if (abcd & e) != 0 {continue; }
                    visualize_word(&a);
                    println!(" {}", decode_word(&a, &words_five));
                    visualize_word(&b);
                    println!(" {}", decode_word(&b, &words_five));
                    visualize_word(&c);
                    println!(" {}", decode_word(&c, &words_five));
                    visualize_word(&d);
                    println!(" {}", decode_word(&d, &words_five));
                    visualize_word(&e);
                    println!(" {}", decode_word(&e, &words_five));
                    println!("...\n");
                } //m
            } //l
        } //k
    } //j

}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String>{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[allow(dead_code)]
fn enc_head(vector: &Vec<u32>){
    visualize_word(&vector[0]);
    visualize_word(&vector[1]);
    visualize_word(&vector[2]);
}

#[allow(dead_code)]
fn vec_head(vector: &Vec<String>){
    println!("{}", vector[0]);
    println!("{}", vector[1]);
    println!("{}", vector[2]);
    println!("{}", vector[3]);
}

#[allow(dead_code)]
fn experiment(){
    println!("\nExperiment:");
    let words = vec!["lack", "leck", "lick", "lock", "luck", "click"];
    for word in &words {
       let enc = encode_word(word);
       visualize_word(&enc);
    }
    println!("----");

    let mut enc_words: Vec<u32> = words.iter().map(|x| encode_word(x)).collect();
    enc_words.sort();
    enc_words.dedup();
    println!("{:?}", enc_words);

    /*
    let ex_enc = encode_word("cajas");
    visualize_word(ex_enc); 
    println!("{}", words[2]);
    */
}

fn encode_word(word: &str) -> u32{
    let mut i_enc: u32 = 0;
    for c in word.chars(){
        let c_num: u32 =  c as u32 - 'a' as u32;
        i_enc = i_enc | (1 << (25-c_num)); 
    }
    return i_enc;
}
fn visualize_word(i_enc: &u32){
    let ascii_a: u32 = 'a' as u32;
    let mut n_char: [char; 26] = [' '; 26];
    let mut a_char: [char; 26] = ['-'; 26];

    for i in 0..26 {
        if i_enc & (1 << i) > 0 { 
            n_char[25-i] = '1';
            a_char[25-i] = char::from_u32(ascii_a + ((25-i) as u32)).unwrap()
        }
    }
    //println!("{}", n_char.iter().collect::<String>() );
    print!("{}", a_char.iter().collect::<String>() );
    io::stdout().flush().unwrap();
}

fn decode_word(i_enc: &u32, words: &Vec<String>) -> String{
    let dec_word = words
        .iter()
        .filter(|w| encode_word(w) == *i_enc)
        .collect::<Vec<_>>();

    //String and &str madness
    let dec_str: Vec<&str> = dec_word.iter().map(|s| &***s).collect();
    let dec_join = dec_str.join("/");

    return dec_join;
}

