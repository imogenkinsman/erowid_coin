/*
"Erowid Coin" is a markov chain generator for tweeting about the unholy marriage of erowid trip
reports + cryptocurrency - it's build using local text files.

Usage: erowidcoin <directory> <number of tweets (optional)>
*/

pub mod markov_chain;

use std::env;
use markov_chain::MarkovChain;
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut num_tweets: i32 = 1;

  if args.len() < 2 {
    println!("usage: erowidcoin <text directory> <number of tweets>");
    return;
  }

  if args.len() == 3 {
    let integer = args[2].parse::<i32>();
    num_tweets = match integer {
      Ok(val) => val,
      Err(error) => {
        println!("could not parse number of tweets: {}", error);
        return;
      },
    };
  }

  let directory = Path::new(&args[1]);

  // is there some way to avoid having to pass mut all the way down :|
  let mut mchain = MarkovChain::new();
  let tweets = mchain.create_tweets(directory, num_tweets);

  for tweet in tweets.iter() {
    println!("{}\n", tweet);
  }
}
