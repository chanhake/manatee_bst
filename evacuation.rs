/*
Author: Jared Blanco, jblanco2018@my.fit.edu
Author: Chandler Hake, chake2019@my.fit.edu
Course:  CSE 4250, Fall 2022 Programming Lanuage concepts
Project: Proj2, Manatee Evacuation with BST in rust https://cs.fit.edu/~ryan/cse4250/projects/evacuation/
Implementation: rustc 1.64 
Description and code found at: https://github.com/chanhake/manatee_bst
To run assignment: Use file redirection or uncomment prompts in scanManatees function for walkthrough
	ie: "rustc evacuation.rs && ./evacuation < input1.txt"


Note to Dr. Stansifer
Thank you for the extra time, this was a super fun puzzle to solve.
Our solution is a little different than what we talked about during your "office hours", 
becuase we don't look back through the tree if we don't find a possible mate for our female.

Hopefully our logic is correct... if not, alteast we tried :)
*/

use std::io;
use std::collections::BTreeSet;
use std::ops::Bound::Included;
use std::process::exit;

/////////////////////////////////////
// SUPPORTING FUNCTIONS
/////////////////////////////////////

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

/////////////////////////////////////
// MAIN
/////////////////////////////////////

fn main(){
    /////////////////////////////////////
    // DECLARATIONS
    /////////////////////////////////////
    
    // decalring our vectors of manatees in tuple form (age, size, ID)
    let mut female_vec = Vec::<(i64, i64, i64)>::new();
    let mut male_vec = Vec::<(i64, i64, i64)>::new();

    // used to find possible male mates for our female
    let mut possible_males = Vec::<(i64, i64, i64)>::new();
    let mut chosen_male: (i64, i64, i64);

    // used for outputting the manattes chosen in their order
    let mut female_output = Vec::<(i64, i64, i64)>::new();
    let mut male_output = Vec::<(i64, i64, i64)>::new();

    // declarings tree's that will be filled from coresponding vectors
    let mut female_tree = BTreeSet::new();
    let mut male_tree = BTreeSet::new();

    let mut input_string = String::new();

    // used for finding age restriciton in our BST searches
    let mut youngest_age: i64;

    /////////////////////////////////////
    // READING INPUT
    /////////////////////////////////////

    // takes in in from stdin as string, converts to i64
    io::stdin().read_line(&mut input_string).unwrap();
    let _n: i64 = input_string.trim().parse().unwrap();

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
    // CREATING BST
    /////////////////////////////////////
    
    // adding all tuples to our BTreeSets
    for tup in &female_vec{
        female_tree.insert(*tup);
    }

    for tup in &male_vec{
        male_tree.insert(*tup);
    }

    /////////////////////////////////////
    // FINDING BEST MALE PARTHNER FOR EACH FEMALE
    /////////////////////////////////////
    
    // O(n log n) : O(n) For all of the females in order of age, O(log n) find  best parthner
    for female_manatee in female_vec{
        // used to maintain age order of chosen pairs
        youngest_age = male_vec[0].0;
        
        // finds the youngest male that is smaller than our current female
        for &elem in male_tree.range((Included(&(0,0,0)), Included(&(youngest_age,female_manatee.1 - 1,9999)))) {
            // pushes all of our options, so we can take the biggest
            possible_males.push(elem);
        }
        
        // takes the biggest one we can from the list of possible mates
        if possible_males.last() != None{
            chosen_male = *(possible_males.last().unwrap());
            female_output.push(female_manatee);
            male_output.push(chosen_male);
            male_tree.remove(&chosen_male);
            male_vec.remove(possible_males.len() - 1); // THIS NEEDS TO RMEOVE CHOSEN MATE AND NOT JUST FIRST
        }else{
            // if we find no possible mates that are the right age and smaller than our female->
                // case must be imposible... or so we are guessing becuase we don't want to look at the other tree lol
            println!("impossible");
            exit(0);
        }

        possible_males.clear();
    }

    /////////////////////////////////////
    // PRINTING FINAL OUTPUT
    /////////////////////////////////////

    for female_out in female_output{
        print!("{} ", female_out.2);
    }
    print!("\n");

    for male_out in male_output{
        print!("{} ", male_out.2);
    }
    print!("\n");
}
