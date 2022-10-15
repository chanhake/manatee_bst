// Jared Blanco and Chandler Hake experimental Code
use std::io;
use std::collections::BTreeSet;

// reads in stdin, convert input to our vector of tuple format
// uses pass by reference logic to change values of vector declared in main
fn input_vector(vec: &mut Vec<(i64, i64, i64)>){
    // used later for type casting string to i64
    let mut current_age: i64;
    let mut current_size: i64;

    // takes in age strings and parses
    let mut ages_string = String::new();
    io::stdin().read_line(&mut ages_string).unwrap();
    let ages = ages_string.trim().to_owned();

    // takes in sizes strings and parses
    let mut sizes_string = String::new();
    io::stdin().read_line(&mut sizes_string).unwrap();
    let sizes = sizes_string.trim().to_owned();

    // combines both parsed strings of ages and sizes to give us manatee data
    let mut id_num = 1;
    for x in ages.split_whitespace().zip(sizes.split_whitespace()){
        current_age  = x.0.parse::<i64>().unwrap();
        current_size = x.1.parse::<i64>().unwrap();
        vec.push((current_age, current_size, id_num));
        id_num = id_num + 1;
    }
}


fn main(){
    /////////////////////////////////////
    // DECLARATIONS
    /////////////////////////////////////
    
    // decalring our vectors of manatees in tuple form (id, age, size)
    // later going to need to change this to be (age, size, id) for BTreeSet
    let mut female_vec = Vec::<(i64, i64, i64)>::new();
    let mut male_vec = Vec::<(i64, i64, i64)>::new();

    // declarings tree's that will be filled from coresponding vectors
    let mut female_tree = BTreeSet::new();
    let mut male_tree = BTreeSet::new();

    let mut input_string = String::new();
    let n: i32;


    /////////////////////////////////////
    // READING INPUT
    /////////////////////////////////////

    // takes in in from stdin as string, converts to i32
    io::stdin().read_line(&mut input_string).unwrap();
    n = input_string.trim().parse().unwrap();
    println!("n = {n}");

    // scans in the input to tuples
    input_vector(&mut female_vec);
    input_vector(&mut male_vec);


    /////////////////////////////////////
    // SORT FUNCTIONS FOR VECTORS
    /////////////////////////////////////

    // Female (a:age, d:size, _x:index)
    // if age is == then sort by the size else just sort by age
    female_vec.sort_by(|(a, d, _x), (b, c,_y)| {
        if a == b{
            d.cmp(&c)
        }
        else{
            a.cmp(&b)
        }
    });
    
    // Male (a:age, d:size, _x:index)
    male_vec.sort_by(|(a, d, _x), (b, c,_y)| {
        if a == b{
            d.cmp(&c)
        }
        else{
            a.cmp(&b)
        }
    });

    /////////////////////////////////////
    // DISPLAYING DATA IN VECTORS
    /////////////////////////////////////

    println!("Printing female manatees");
    for tup in female_vec{
        println!("{:?}", tup);
    }

    println!("Printing male manatees");
    for tup in male_vec{
        println!("{:?}", tup);
    }

    female_tree.insert((1, 4, 1));
    male_tree.insert((2, 1, 4));

    /////////////////////////////////////
    // FUN BST TESTING
    /////////////////////////////////////
    
    /*
    // adding all tuples to our BTreeSets
    for tup in female_vec{
        female_tree.insert(tup);
    }

    for tup in male_vec{
        male_tree.insert(tup);
    }
    */


    // testing with sorted input1 in this format (age, size, ID)
    female_tree.insert((1, 4, 1));
    female_tree.insert((2, 3, 2));
    female_tree.insert((2, 3, 3));
    female_tree.insert((3, 2, 4));

    male_tree.insert((1, 2, 1));
    male_tree.insert((1, 3, 2));
    male_tree.insert((2, 2, 3));
    male_tree.insert((2, 1, 4));

    println!("female_tree");
    for female_manatee in &female_tree {
        let closest_mate = male_tree.range(..female_manatee).next_back().unwrap();
        println!("{:?} matched with {:?}", female_manatee, closest_mate);
        // need to delete closest_mate and female_manatee here
    }

}
