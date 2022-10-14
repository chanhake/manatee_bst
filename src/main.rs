use std::collections::BTreeMap;
use std::convert::TryInto;
use std::process::exit;
//use std::iter::FromIterator;

fn main() {
    let mut male_iter_bst = BTreeMap::new();
    let mut female_iter_bst = BTreeMap::new();
    let mut input_string = String::new();
    let mut female_vec = Vec::<(i64, i64, i64)>::new();
    let mut male_vec = Vec::<(i64, i64, i64)>::new();
    //READ IN SIZE
    std::io::stdin()
    .read_line(&mut input_string)
    .expect("Failed to read string");

    //TRIM and convert to usize
    let columns: usize = input_string
    .trim()
    .parse()
    .expect("Wanted Number instead");

    for _line in 0..2{

        //AGE of manatee
        let mut age_str= String::new();

        //Read string from stdin
        std::io::stdin()
        .read_line(&mut age_str)
        .expect("Failed to read string");

        //Get rid of newline in string
        age_str.pop();

        //Split string by space and map
        let age:Vec<i64> = age_str
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

        //Column Error Check
        if age.len()!= columns{
            println!("Incorrect Input!");
            exit(0);
        }

        //SIZE of manatee
        let mut size_string= String::new();

        //Read string from stdin
        std::io::stdin()
        .read_line(&mut size_string)
        .expect("Failed to read string");

        //Get rid of newline in string
        size_string.pop();

        //Split string by space
        let size:Vec<i64> = size_string
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

        //Column Error Check
        if size.len()!= columns{
            println!("Incorrect Input!");
            exit(0);
        }
        
        // //Insert data to BST
        if _line == 0{
            for _i in 0..columns{
                female_vec.push(((_i+1).try_into().unwrap(), age[_i], size[_i]));
            }
        }
        else{
            for _i in 0..columns{
                male_vec.push(((_i+1).try_into().unwrap(), age[_i], size[_i]));
            }
        }
    }
    
    // //Convert male bst to vector
    female_vec.sort_by(|(_x, a, d), (_y, b,c)| {
        if a == b{
            d.cmp(&c)
        }
        else{
            a.cmp(&b)
        }
    });
    
    // //Convert female bst to vector
    male_vec.sort_unstable_by(|(_x, a, d), (_y, b,c)| {
        if a == b{
            d.cmp(&c)
        }
        else{
            a.cmp(&b)
        }
    });
    

    //Insert to BTreeMap you dont have to use this if you want to implement your own
    for _i in 0..columns{
        female_iter_bst.insert(_i, (female_vec[_i].0, female_vec[_i].1, female_vec[_i].2));
    }
    for _i in 0..columns{
        male_iter_bst.insert(_i, (male_vec[_i].0, male_vec[_i].1, female_vec[_i].2));
    }

    println!("\nPrinting elements");
    println!("{:?}", female_vec);
    println!("{:?}", male_vec);
    
}
