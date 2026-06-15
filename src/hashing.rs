use std::collections::HashMap;

pub struct HashQues;

impl HashQues {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{

        let mut map: HashMap<i32, i32> = HashMap::new();

        for(i , &num) in nums.iter().enumerate(){
            let compliment: i32 = target - num;
            if map.contains_key(&compliment){
                return vec![*map.get(&compliment).unwrap(), i as i32];
            }
            map.insert(num,i as i32);
        }

        return vec![-1,-1];

    }

    pub fn contains_duplicate(nums: Vec<i32>)-> bool{

        use std::collections::HashSet;

        let mut set: HashSet<i32> = HashSet::new();

        for &num in nums.iter(){

            if !set.insert(num){

                return true;

            }

        }

        return false;

    }


   

    pub fn valid_anagram(s: String, t: String) -> bool{

        if s.len() != t.len() {return false;}

        let mut counts: HashMap<char, i32> = HashMap::new();

        for c in s.chars(){
            *counts.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            *counts.entry(c).or_insert(0) -=1;
        }

        counts.values().all(|&v| v == 0)

    }

    pub fn group_anagram(s: Vec<String>) -> Vec<Vec<String>>{

        let mut groups: HashMap<String,Vec<String>> = HashMap::new();

        for c in s{
            let mut char: Vec<char> = c.chars().collect();
            char.sort();
            let key :String= char.iter().collect();

            groups.entry(key).or_insert_with(Vec::new).push(c);



        }

        groups.into_values().collect()
    }

    
}


