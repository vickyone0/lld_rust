use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

pub struct Heap;

impl Heap {

    pub fn find_kth_largest(nums:  Vec<i32>, k:i32) -> i32 {


        let mut heap :BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for num in nums{
            heap.push(Reverse(num));

            if heap.len() > k as usize {
                heap.pop();
            }

        }

        heap.peek().unwrap().0


    }

    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut count = HashMap::new();

        for num in &nums {

            *count.entry(*num).or_insert(0)+=1;
        }

        let  mut heap : BinaryHeap<Reverse<(i32,i32)>> = BinaryHeap::new();


        for (&val, &freq) in &count {

            heap.push(Reverse((freq,val)));

            if  heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut result = Vec::new();

        while let Some(Reverse((_freq, val))) = heap.pop() {

            result.push(val);

        }

        result

    }

}