use std::env;
use std::process;

#[derive(Debug)]
struct Arg
{
    double: bool,
    triple: bool,
    word: String,
}

fn main()
{
    let arg = get_arg();
	let mut score = 0;
	let mut point = 1;
	for words in arg.word.chars()
	{
		if words == 'd' || words == 'g'
        {
            point = 2;
        }
        else if words == 'b' || words == 'c' || words == 'm' || words == 'p'
        {
            point = 3;
        }
        else if words == 'f' || words == 'h' || words == 'v' || words == 'w' || words == 'y'
        {
            point = 4;
        }
        else if words == 'k'
        {
            point = 5;
        }
        else if words == 'j' || words == 'x'
        {
            point = 8;
        }
        else if words == 'q' || words == 'z'
        {
            point = 10;
        }
        println!("{}: {}", words, point);
        score += point;
        point = 1;
	}
	if arg.double == true
	{
		score = score * 2;
		println!("x2");
	}
	if arg.triple == true
	{
		score = score * 3;
		println!("x3");
	}
	println!("Score = {}", score);
}

fn get_arg() -> Arg
{
    let mut arg = Arg
    {
        double: false,
        triple: false,
        word: String::new(),
    };

    for arges in env::args().skip(1)
    {
        if arges == "--double" || arges == "-d"
        {
            arg.double = true;
        }
        else if arges == "--triple" || arges == "-t"
        {
            arg.triple = true;
        }
        else if arges == "-dt" || arges == "-td"
        {
            arg.double = true;
            arg.triple = true;
        }
        else
        {
            arg.word = arges;
        }
    }
    
    if arg.word.is_empty()
    {
        eprintln!("The word to score is missing.");
        process::exit(1);
    }
    
    arg
}