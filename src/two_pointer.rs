use core::num;

pub struct Twopointer;

impl Twopointer {


    pub fn valid_palindrome(s: String) -> bool {
        
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() { return true; }
        let mut left = 0;
        let mut right = chars.len() - 1;


        while left < right {
                 
                 while left < right && !chars[left].is_alphanumeric() { left +=1;}
                 while left < right && !chars[right].is_alphanumeric() { right -=1;}

                 if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                    return false;
                 }

                 left +=1;
                 right -=1;

        }

        return true;


    }
    

    pub fn two_sum(nums : Vec<i32>, target : i32) -> Vec<i32>{

        let mut left = 0;
        let mut right = nums.len() -1;

        while left < right {
            let sum = nums[right] + nums[left];

            if sum == target {
                return vec![(left+1) as i32 , (right + 1) as i32];
            }
            else if sum > target {
                right -=1;
                
            }else {
                left +=1;
            }
        }

        return vec![ -1, -1];
    }

    pub fn valid_parentheses(s: String)-> bool{

        let mut stack: Vec<char> = Vec::new();

        for c in s.chars(){

            match c {
                '(' | '{' | '[' => stack.push(c),
                ')' => {
                    if stack.pop()!= Some('(') {return false; }
                },
                '}' => {
                    if stack.pop() != Some('{') { return  false;}
                },
                ']' => {
                    if stack.pop() != Some('[') { return false;}
                },
                _ => {},
            }
        }
        return stack.is_empty();
    }
    

}


struct MinStack{
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {

        MinStack { 
            stack: Vec::new(),
            min_stack: Vec::new()
        }

    }

    fn push(&mut self, val: i32){

        self.stack.push(val);

        let min_num = if self.min_stack.is_empty() {
            val
        } else {
            std::cmp::min(val,*self.min_stack.last().unwrap())
        };

        self.min_stack.push(min_num);

    }

    fn pop(&mut self) {
        self.stack.pop();

        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
    *self.stack.last().unwrap()
   }

   fn get_min(&self) -> i32{
    *self.min_stack.last().unwrap()
   }

}