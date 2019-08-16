use std::collections::HashMap;
use std::io;

fn main() {
    assorted_values();
    pig_latin("test");
    sim_company();
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

fn pig_latin(input: &str) {
    let vowels = ['a', 'e', 'y', 'o', 'u'];

    let mut is_vowel = false;
    for c in input.chars() {
         is_vowel = match vowels.iter().find(|&&x| x == c) {
             Some(_value) => true,
             None => false
         };
         break;
    }

    let mut strng: String;
    if is_vowel {
        strng = String::from(input) + "hay";
    } else {
        strng = String::from(&input[1..]) + &input[0..1] + "ay";
    }

    println!("pig latin for {} is {}", input, strng);
}

fn sim_company() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut command = String::new();
        println!("wait for user command");
        io::stdin().read_line(&mut command).expect("An error happened");

        let tokens = tokenize(&command);
        let cmd = match tokens.get(0) {
            Some(value) => value,
            None => {
                println!("malformed command {}", command);
                continue;
            }
        };

        match &cmd[..] {
            "Add" => {
                let employee = match tokens.get(1) {
                    Some(value) => value,
                    None => {
                        println!("malformed command {}", command);
                        continue;
                    }
                };

                let department = match tokens.get(3) {
                    Some(value) => value,
                    None => {
                        println!("malformed command {}", command);
                        continue;
                    }
                };

                let employees = departments.entry(department.to_string()).or_insert(Vec::new());
                employees.push(employee.to_string());
            },
            "Show" => {
                let department = match tokens.get(1) {
                    Some(value) => value,
                    None => {
                        println!("malformed command {}", command);
                        continue;
                    }
                };

                let employees = departments.entry(department.to_string()).or_insert(Vec::new());
                employees.sort();

                for employee in employees {
                    println!("Employee {}", employee);
                }

            },
            "All" => {

            },
            _ => {
                println!("malformed command {}", command);
                continue;
            }
        };
    }
}

fn tokenize(input: &String) -> Vec<String> {
    let split = input.split_whitespace();
    return split.map(|val| String::from(val)).collect();
}