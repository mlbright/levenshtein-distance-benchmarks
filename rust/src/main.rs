mod levenshtein_distance;

use levenshtein_distance::levenshtein_distance;

fn main() {
    let lines: Vec<&str> = include_str!("../../sample.txt").split('\n').collect();

    let benchmark = || {
        for _ in 0..10000 {
            let mut last_value = "";
            for line in &lines {
                levenshtein_distance(last_value, line);
                last_value = line;
            }
        }
    };

    use std::time::Instant;
    let now = Instant::now();

    {
        benchmark();
    }

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    print!("{}", sec);

    // check
    let answers: Vec<String> = (0..lines.len()-1)
        .map(|i| levenshtein_distance(lines[i], lines[i+1]))
        .map(|dist| dist.to_string())
        .collect();
    eprintln!("{}", answers.join(","));
}
