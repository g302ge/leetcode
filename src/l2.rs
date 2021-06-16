use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let (mut left, mut right, mut carry) = (l1, l2, 0);
        let mut current = dummy.as_mut();
        while left.is_some() || right.is_some() {
            let mut sum = carry;
            if let Some(v) = left {
                sum += v.val;
                left = v.next;
            }
            if let Some(v) = right {
                sum += v.val;
                right = v.next;
            }
            carry = sum / 10;
            if let Some(c) = current {
                c.next = Some(Box::new(ListNode::new(sum % 10)));
                current = c.next.as_mut();
            }
        }
        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::l2;
    use crate::ListNode;
    #[test]
    fn leetcode2() {
        let left = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let right = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let mut calculated = l2::Solution::add_two_numbers(left, right);
        let mut result = vec![];
        let mut current = calculated.as_mut();

        while let Some(cur) = current {
            result.push(cur.val);
            current = cur.next.as_mut();
        }

        assert_eq!(result, vec![7, 0, 8]);
    }
}
