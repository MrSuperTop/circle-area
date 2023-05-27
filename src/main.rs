use rand::prelude::*;
use rayon::iter::repeat;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::time::Instant;

fn read_stdin(query: &str) -> String {
    let mut user_input = String::new();
    let additional_part = "\n>>> ";

    print!("{}{}", query, additional_part);

    let stdin = io::stdin();
    let _ = io::stdout().flush();

    stdin
        .read_line(&mut user_input)
        .expect("Could not read from stdin");

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input.to_string();
}

fn count_in_the_circle(total_points_count: u64, r: f64) -> u64 {
    let mut rng = rand::thread_rng();
    let mut points_in_the_circle: u64 = 0;

    for _ in 0..total_points_count {
        let (x, y) = (rng.gen_range(-r..=r), rng.gen_range(-r..=r));
        if (x.powi(2) + y.powi(2)).sqrt() <= r {
            points_in_the_circle += 1;
        }
    }

    return points_in_the_circle;
}

fn approximate_area(number_of_points: u64, r: f64, chunks_number: Option<usize>) -> f64 {
    let concurrency_count = chunks_number.unwrap_or(8);
    if number_of_points % concurrency_count as u64 != 0 {
        panic!("The total number of points has to be divisible without any remained by the threads count");
    }

    let single_thread_points = number_of_points / concurrency_count as u64;
    let thread_point_counts = repeat(single_thread_points).take(concurrency_count);

    let points_in_the_circle: u64 = thread_point_counts
        .map(|points| count_in_the_circle(points, r))
        .sum();

    let d = r * 2.0;
    let approximated_area = d.powi(2) * (points_in_the_circle as f64 / number_of_points as f64);

    return approximated_area;
}

fn main() -> io::Result<()> {
    let r: f64 = read_stdin("Input the radius of the circle")
        .parse()
        .unwrap();

    let number_of_points: u64 =
        read_stdin("Input the number of points to randomly distribute inside the circle")
            .parse()
            .unwrap();

    let chunks_number: usize =
        read_stdin("Input the number of chucks to split the computation into")
            .parse()
            .unwrap();

    let real_area = PI * r.powi(2);

    let start = Instant::now();
    let approximated_area = approximate_area(number_of_points, r, Some(chunks_number));
    let elapsed_millis = start.elapsed().as_millis();

    println!(
        "Approximated area of the circle (took {} ms): {}\nReal area: {}",
        elapsed_millis, approximated_area, real_area
    );

    Ok(())
}
