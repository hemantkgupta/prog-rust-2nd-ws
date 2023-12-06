use rayon::prelude::*; 

// https://codedamn.com/news/rust/advanced-concurrency-rust-exploring-parallelism-rayon

pub fn par_rayon_work(){
    let names = ["Alice", "Bob"]; 
    println!("The size is: {}", par_sum_of_char_len(&names));
    parallel_map();

}

fn parallel_map(){
    let numbers = vec![1, 2, 3, 4, 5];

    // Type is required - Seqeuential
    let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("{:?}", squares);

    // Type is required - parallel
    let squares: Vec<_> = numbers.par_iter().map(|x| x * x).collect();
    println!("{:?}", squares);

    // Filter
    let even_numbers: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even_numbers);

    let even_numbers: Vec<_> = numbers.par_iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even_numbers);

    // Map reduce
    let sum: i32 = numbers.iter().cloned().reduce(|a, b| a + b).unwrap();
    println!("Sum: {}", sum);

    let sum: i32 = numbers.iter().cloned().reduce(|a, b| a + b).unwrap();
    println!("Sum: {}", sum);

    // Parallel sort
    let mut numbers = vec![5, 4, 3, 2, 1];
    numbers.par_sort();
    println!("{:?}", numbers);

    let n = 20;
    let fib = parallel_fibonacci(n);
    println!("Fibonacci({}) = {}", n, fib);


}

fn parallel_fibonacci(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        let (a, b) = rayon::join(|| parallel_fibonacci(n - 1), || parallel_fibonacci(n - 2));
        a + b
    }
}

fn par_sum_of_char_len(list: &[&str]) -> usize { 
    list.par_iter().map(|word| word.len()).sum() 
} 