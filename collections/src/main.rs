use std::collections::HashMap;

fn main() {
    assorted_values();
}

fn assorted_values() {
    let mut v = vec![1,3,4,123,1];

    let mut sum = 0;
    for i in &v {
        sum += i;
    }

    println!("the mean is {}", (sum as f32)/(v.len() as f32) );

    let mut outer = 0;
    while outer < v.len() {
        let mut inner = v.len() - 1;
        while inner > outer {
            if v[inner] < v[inner - 1] {
                let tmp = v[inner - 1];
                v[inner - 1] = v[inner];
                v[inner] = tmp;
            }
            inner -= 1;
        }
        outer += 1;
    }

    println!("the sorted array looks like");
    for i in &v {
        println!("{}", i);
    }

    println!("the median is {}", v[v.len()/2]);

    let mut counter = HashMap::new();
    let mut most = [0, 0];
    for i in &v {
        let count = counter.entry(i).or_insert(0);
        *count += 1;

        if *count > most[1] {
            most = [*i, *count];
        }
    }

    println!("the most frequent number is {}", most[0]);

}