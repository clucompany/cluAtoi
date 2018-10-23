
extern crate cluatoi;
use cluatoi::Atoi;
use cluatoi::AtoiErr;

pub fn main() {
	let num = usize::atoi(b"-1245");	// WHY ERR? THIS USIZE !!! INDEX[0] = '-'
	assert_eq!(num, Result::Err(AtoiErr::ByteUnk(b'-')));
	print_usize(num);
	
	
	let num = usize::atoi_stop(b"1245T", b'T');
	assert_eq!(num, Result::Ok(1245));
	print_usize(num);
	
	
	let num = usize::atoi_iter(b"1245".iter());
	assert_eq!(num, Result::Ok(1245));
	print_usize(num);
	
}


#[inline]
pub fn print_usize(result_parse_num: Result<usize, cluatoi::AtoiErr>) {
	println!("{:?}", result_parse_num);
}
