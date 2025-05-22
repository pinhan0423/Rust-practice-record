fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
   let mut carry = 1;
   let mut ans = Vec::new(); 

   for i in (0..digits.len()).rev(){
    let sum = digits[i] + carry;
    ans.push(sum%10);
    carry = sum / 10;
   }

   if carry == 1{
    ans.push(1);
   }
   ans.reverse();
   ans
}

fn main(){
    let input = vec![9,9,9];
    let result = plus_one(input);
    println!("plus one result: {:?}", result);
    
}
