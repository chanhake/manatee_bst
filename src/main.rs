use std::collections::BTreeMap;
use std::process::exit;

fn main() {
    let mut iter_bst = BTreeMap::new();
    let mut input_string = String::new();

    //READ IN SIZE
    std::io::stdin()
    .read_line(&mut input_string)
    .expect("Failed to read string");

    //TRIM
    let columns: usize = input_string
    .trim()
    .parse()
    .expect("Wanted Number instead");

    for _line in 0..2{

        //AGE
        let mut age_str= String::new();
        std::io::stdin()
        .read_line(&mut age_str)
        .expect("Failed to read string");
        age_str.pop();
        let split1 = age_str.split(" ");
        let age:Vec<&str> = split1.collect(); 

        //Column Error Check
        if age.len()!= columns{
            println!("Incorrect Input!");
            exit(0);
        }

        //SIZE
        let mut str2= String::new();
        std::io::stdin()
        .read_line(&mut str2)
        .expect("Failed to read string");
        str2.pop();
        let split2 = str2.split(" ");
        let size:Vec<&str> = split2.collect();
        
        //Column Error Check
        if size.len()!= columns{
            println!("Incorrect Input!");
            exit(0);
        }
        //INSERT
        for i in 0..columns{
            //How do we want to put data in the tree?
             iter_bst.insert(i, (age[i].trim().parse::<i64>(), size[i].trim().parse::<i64>()));
        }

    }
    
}
