fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5];

    let combined: Vec<i32> = a.iter().chain(b.iter()).cloned().collect();

    println!("{:?}", combined); 
}

// output : [1, 2, 3, 4, 5]

fn main() {
    let data = vec![1, 2, 3];
    let mut it = data.iter();

    println!("{:?}", it.next()); // Some(1)
    println!("{:?}", it.next()); // Some(2)
    println!("{:?}", it.next()); // Some(3)
    println!("{:?}", it.next()); // None
}



fn print_items<I: IntoIterator<Item = i32>>(items: I) {
    for item in items {
        println!("{}", item);
    }
}

fn main() {
    print_items(vec![10, 20, 30]); // vector
    print_items([40, 50, 60]);     // array
}

fn main() {
    let data = vec![7, 8, 9];
    let mut it = data.iter();

    println!("{:?}", it.next()); // Some(7)
    println!("{:?}", it.next()); // Some(8)

    // iterator tetap lanjut dari posisi terakhir
    println!("{:?}", it.next()); // Some(9)
}

fn main() {
    let nums = vec![3, 6, 9];

    let it = nums.iter();

    for n in it {
        println!("angka: {}", n);
    }
}

fn main() {
    let letters = vec!["a", "b", "c"];
    let mut it = letters.iter();

    while let Some(l) = it.next() {
        println!("ambil: {}", l);
    }
}

use rayon::prelude::*;

fn main() {
    let a = vec![10, 20, 30];
    let b = vec![40, 50, 60];

    // gabungkan dulu
    let combined: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();

    // diproses paralel
    let doubled: Vec<i32> = combined
        .into_par_iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled);
}

use rayon::prelude::*;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // Proses paralel: setiap elemen dikalikan 2
    let result: Vec<i32> = data
        .par_iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", result);
}
