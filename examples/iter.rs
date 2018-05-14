
extern crate cluatoi;

use cluatoi::Atoi;
use cluatoi::AtoiErr;

fn main() {
	let array = b"1024!18446744073709551610!-1!X";
	let mut array_iter = array.iter();
    
    
	let num_0 = u64::atoi_iter_wait_stop(&mut array_iter, b'!').unwrap(); 
	assert_eq!(num_0, 1024u64);
	//1024u64
    
    
	let num_1 = u64::atoi_iter_wait_stop(&mut array_iter, b'!').unwrap(); 
	assert_eq!(num_1, 18446744073709551610u64);
	//18446744073709551610u64
    
    
	let num_err = usize::atoi_iter_wait_stop(&mut array_iter, b'!');
	assert_eq!(num_err, Result::Err(AtoiErr::ByteUnk(b'-')));
	//ERROR, ISIZE VALUE + USIZE TYPE
	//Err(ByteUnk(45))
    
    
	let end_byte = array_iter.next().unwrap();
	assert_eq!(end_byte, &b'X');
	//X
	
	assert_eq!(array_iter.next(), None);
	
	
    
	println!("{}!{}!{:?}!{}", num_0, num_1, num_err, end_byte);
	//1024!18446744073709551610!Err(ByteUnk(45))!88
}
