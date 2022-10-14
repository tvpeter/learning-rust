/*
    Topic: Arrays and slices

    Requirements:
    - print pairs of numbers and their sums as they are stremed from a data source
    - if only one number is received, then print "unpaired value: v" where v is the value
    - if no numbers are received, print "data stream complete"

    Notes:
    - A simulated data stream is already configured in the code
    - see the stdlib docs for "chunks" method on "slice" for more info
*/

fn data() -> &'static [u64] {
    &[5, 5, 4, 4, 3, 3, 1]
}

//second method
fn process_chunck(data: &[u64]) {
    match data {
        [lhs, rhs] => println!("{} + {} = {}", lhs, rhs, (lhs + rhs)),
        [single] => println!("unpaired value: {}", single),
        [] => println!("Data stream complete"),
        [..] => unreachable!("chunk size should be at most 2"),
    }
}

fn main() {
    //stream is an iterator of Option<&[u64]>
    let stream = data().chunks(2);

    //start of method 1
    for slice in stream {
        let mut sum = 0;
        for elem in slice.iter() {
            sum += elem;
        }

        if slice.len() == 1 {
            println!("unpaired value: {:?}", slice);
        } else {
            println!("slice: {:?}, sum: {:?}", slice, sum);
        }
        //end of method one
        //start of second method
        // process_chunck(slice);
    }
}
