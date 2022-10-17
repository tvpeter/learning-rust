/*
--HACKERRANK
An array is a type of data structure that stores elements of the same type in a contiguous block of memory. In an array, , of size , each memory location has some unique index,  (where ), that can be referenced as  or .

Reverse an array of integers.

Note: If you've already solved our C++ domain's Arrays Introduction challenge, you may want to skip this.

Example

Return .

Function Description

Complete the function reverseArray in the editor below.

reverseArray has the following parameter(s):

int A[n]: the array to reverse
Returns

int[n]: the reversed array
Input Format

The first line contains an integer, , the number of integers in .
The second line contains  space-separated integers that make up .

Constraints

Sample Input 1


1 4 3 2

Sample output
2341

*/
fn reverseArray(a: &[i32]) -> Vec<i32> {
    let mut newVec = Vec::new();
    let mut a_length = a.len();
    
    while a_length > 0 {
        newVec.push(a[a_length - 1]);
        a_length -= 1;
    }
    newVec
}

fn main(){
    let array = [1, 4, 3, 2];

    let result = reverseArray(&array);

    println!("{:?}", result);
}
