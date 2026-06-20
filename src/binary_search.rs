

pub struct BinarySearch;

impl BinarySearch {

    pub fn binary_search (nums: Vec<i32>, target: i32) -> i32 {

        let mut left :usize = 0;
        let mut right :usize = nums.len() -1;

        while left <= right {

            let mid = left + (right - left)/2;

            if nums[mid] == target {
                return mid as i32;
            }else if nums[mid] < target {
                left = mid +1;
                
            }else {
                if mid == 0 {break;}
                right = mid -1;
            }
        }

        return -1;
    }

    pub fn fin_min(nums: Vec<i32>) -> i32 {
        let mut left =0;
        let mut right = nums.len()-1;

        while left < right {

            let  mid = left + (right - left )/2;

            if nums[mid] > nums[right] {
                left = mid +1;
            } else {
                right = mid;
            }

        }

        nums[left]
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32{

        let mut left = 0;
        let mut right = nums.len()-1;

        while left <= right {

            let mid = left + (right - left)/2;

            if nums[mid] == target { return mid as i32;}

           if nums[left] <= nums[mid] {

            if nums[left] <= target && target <= nums[mid] {
                if mid == 0 { break;}
                right = mid - 1;  
            } else {
                left = mid + 1; 
}
            
           } else {
               if nums[mid] <= target && target <= nums[right] {
                left = mid+1;
               }else {
                   if mid == 0 { break;}
                   right = mid -1;
               }
           }
        }

        return -1;

        
    }
}