#[derive(Debug)]
struct Arg
{
    double: bool,
    triple: bool,
    word: String,
}

fn get_arg() -> Arg
{
    let mut arg = Arg
    {
        double: false,
        triple: false,
        word: String::new(),
    };
    for e in std::env::args().skip(1)
    {
        if (e == "-d") || (e == "--double")
        {
            arg.double = true;
        }
        else if (e == "-t") || (e == "--triple")
        {
            arg.triple = true;
        }
        else if (e == "-dt") || (e == "-td")
        {
            arg.double = true;
            arg.triple = true;
        }
        else
        {
            arg.word = e;
        }
    }
    if arg.word.is_empty()
    {
        eprintln!("The word to score is missing.");
        std::process::exit(1);
    }
    arg
}

fn main() 
{
    let mut score_sum = 0;
    let arg = get_arg();
    for c in arg.word.chars()
    {
        let score_c: u8;
        if ['d', 'g'].contains(&c)
        {
            score_c = 2;
        }
        else if ['b', 'c', 'm', 'p'].contains(&c)
        {
            score_c = 3;
        }
        else if ['f', 'h', 'v', 'w', 'y'].contains(&c)
        {
            score_c = 4;
        }
        else if ['k'].contains(&c)
        {
            score_c = 5;
        }
        else if ['j', 'x'].contains(&c)
        {
            score_c = 8;
        }
        else if ['q', 'z'].contains(&c)
        {
            score_c = 10;
        }
        else
        {
            score_c = 1;
        }
        println!("{}: {}", c, score_c);
        score_sum += score_c;
    }
    let mut multiply = 1;
    if arg.double
    {
        println!("x2");
        multiply *= 2;
    }
    if arg.triple
    {
        println!("x3");
        multiply *= 3;
    }
    println!("Score = {}", score_sum * multiply);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_get_arg()
    {
        //TODO
    }

}
