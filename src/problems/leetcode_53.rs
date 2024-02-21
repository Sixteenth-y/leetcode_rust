use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut ans = nums[0];

        for val in nums.iter() {
            pre = std::cmp::max(pre + *val, *val);
            ans = ans.max(pre);
        }

        ans
    }
}
