//Copyright 2018 #UlinProject Денис Котляров

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//       http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.


//#Ulin Project 1718
//


//!Parsing the byte sequence of the ascii characters and safely converting them to integers.
//!An open library written in the language of system programming Rust is used to analyze and bring the sequence of ascii bytes to many integer primitives of the language Rust (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128). There is the possibility of using the byte iterators to continue parsing byte arrays without creating a separate byte array or byte slice.
//!Capabilities:
//!    Convert ASCII byte sequences to integer primitives Rust.
//!    Protection against overflow of numbers
//!    Accounting for the features of signed, unsigned primitives
//!    Return of the transfer in case of an error
//!    Using IterByteArray
//!
//!Use:
//!
//!extern crate cluatoi;
//!use cluatoi::Atoi;
//!
//!fn main() {
//!	let array = b"-1245";
//!	let isize = isize::atoi(array).unwrap(); //  -1245isize
//!	let usize = usize::atoi(array).unwrap();  //  AtoiErr(ByteUnk(b'-'))
//!	let array_end = b"1245T";
//!	let my_int = u64::atoi_end(array_end, b'T').unwrap(); //1245u64
//!}




///Type result Atoi
pub type AtoiResult<T> = Result<T, AtoiErr>;

///Parsing the byte sequence of the ascii characters and safely converting them to integers.
pub trait Atoi<T> {
	///Array parsing
	fn atoi<'a>(array: &'a [u8]) -> AtoiResult<T>;
	///Array parsing and stopping on the 'X' character
	fn atoi_end<'a>(array: &'a [u8], end: u8) -> AtoiResult<T>;
	
	///Array parsing using an iterator
	fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult<T>;
	///Array parsing using an iterator and stopping on the 'X' character
	fn atoi_iter_end<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult<T>;
}

///Result trait Atoi
#[derive(Debug, PartialEq, Clone)]
pub enum AtoiErr {
	///Overflow of number
	Overflow,
	///In the byte sequence, an unknown character was used
	ByteUnk(u8),
}


///Unsigned Atoi Macros
macro_rules! atoi {
	(add, $iter:expr) => {{	
		let mut result: Self = 0;
		let mut mul: Self;
		while let Some(a) = $iter.next() {
			if !(*a >= b'0' && *a <= b'9') {
				return Err(AtoiErr::ByteUnk(*a));
			}

			if let Some(s) = result.checked_mul(10) {
				mul = (*a - b'0') as Self;
				if let Some(s) = s.checked_add(mul) {
					result = s;
					continue;
				}
			}
			return Err(AtoiErr::Overflow);
		}
		return Ok(result);
	}};
	(add, $iter:expr, $end:expr) => {{	
		let mut result: Self = 0;
		let mut mul: Self;
		while let Some(a) = $iter.next() {
			if *a == $end {
				break;
			}
			if !(*a >= b'0' && *a <= b'9') {
				while let Some(a) = $iter.next() {
					if *a == $end {
						break;
					}
				}
				return Err(AtoiErr::ByteUnk(*a));
			}

			if let Some(s) = result.checked_mul(10) {
				mul = (*a - b'0') as Self;
				if let Some(s) = s.checked_add(mul) {
					result = s;
					continue;
				}
			}
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::Overflow);
		}
		return Ok(result);
	}};


	(sub, $iter:expr) => {{	
		let mut result: Self = 0;
		let mut mul: Self;
		while let Some(a) = $iter.next() {
			if !(*a >= b'0' && *a <= b'9') {
				return Err(AtoiErr::ByteUnk(*a));
			}

			if let Some(s) = result.checked_mul(10) {
				mul = (*a - b'0') as Self;
				if let Some(s) = s.checked_sub(mul) {
					result = s;
					continue;
				}
			}
			return Err(AtoiErr::Overflow);
		}
		return Ok(result);
	}};
	(sub, $iter:expr, $end:expr) => {{	
		let mut result: Self = 0;
		let mut mul: Self;
		while let Some(a) = $iter.next() {
			if *a == $end {
				break;
			}
			if !(*a >= b'0' && *a <= b'9') {
				while let Some(a) = $iter.next() {
					if *a == $end {
						break;
					}
				}
				return Err(AtoiErr::ByteUnk(*a));
			}

			if let Some(s) = result.checked_mul(10) {
				mul = (*a - b'0') as Self;
				if let Some(s) = s.checked_sub(mul) {
					result = s;
					continue;
				}
			}
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::Overflow);
		}
		return Ok(result);
	}};
}


///Signed Atoi Macros
macro_rules! iatoi {
	(add_event, $a:expr, $result:expr) => {
		if !(*$a >= b'0' && *$a <= b'9') {
			return Err(AtoiErr::ByteUnk(*$a));
		}

		if let Some(s) = $result.checked_mul(10) {
			if let Some(s) = s.checked_add((*$a - b'0') as Self) {
				$result = s;
			}else {
				return Err(AtoiErr::Overflow);
			}
		}else {
			return Err(AtoiErr::Overflow);
		}
	};

	(add_event, $a:expr, $result:expr, $iter:expr, $end:expr) => {
		if $end == *$a {
			return Ok($result);
		}
		if !(*$a >= b'0' && *$a <= b'9') {
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::ByteUnk(*$a));
		}
		if let Some(s) = $result.checked_mul(10) {
			if let Some(s) = s.checked_add((*$a - b'0') as Self) {
				$result = s;
			}else {
				while let Some(a) = $iter.next() {
					if *a == $end {
						break;
					}
				}
				return Err(AtoiErr::Overflow);
			}
		}else {
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::Overflow);
		}
	};
	
	($iter:expr) => {{
		let mut result: Self = 0;
		if let Some(a) = $iter.next() {
			match *a {
				b'-' => {
					//NEGATIVE
					atoi!(sub, $iter)
				},
				b'+' => {
					//POSITIVE
					atoi!(add, $iter)
				},
				_ => {
					iatoi!(add_event, a, result);
					while let Some(a) = $iter.next() {
						iatoi!(add_event, a, result);
					}
				}
			}
		}
		return Ok(result);
	}};


	($iter:expr, $end:expr) => {{
		let mut result: Self = 0;
		if let Some(a) = $iter.next() {
			match *a {
				b'-' => {
					//NEGATIVE
					atoi!(sub, $iter, $end)
				},
				b'+' => {
					//POSITIVE
					atoi!(add, $iter, $end)
				},
				_ => {
					iatoi!(add_event, a, result, $iter, $end);
					while let Some(a) = $iter.next() {
						iatoi!(add_event, a, result, $iter, $end);
					}
				}
			}
		}
		return Ok(result);
	}};
}





macro_rules! checked_impl {
	(u, $t:ty) => {
		impl Atoi<Self> for $t {
			
			fn atoi<'a>(array: &'a [u8]) -> AtoiResult<Self> {
				let mut iter = array.iter();
				atoi!(add, iter)
			}

			//#[inline]
			fn atoi_end<'a>(array: &'a [u8], end: u8) -> AtoiResult<Self> {
				let mut iter = array.iter();
				atoi!(add, iter, end)
			}

			fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult<Self> {
				atoi!(add, iter)
			}

			fn atoi_iter_end<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult<Self> {
				atoi!(add, iter, end)
			}
		}
	};
	(i, $t:ty) => {
		impl Atoi<Self> for $t {
			fn atoi<'a>(array: &'a [u8]) -> AtoiResult<Self> {
				let mut iter = array.iter();
				iatoi!(iter)
			}

			fn atoi_end<'a>(array: &'a [u8], end: u8) -> AtoiResult<Self> {
				let mut iter = array.iter();
				iatoi!(iter, end)
			}
			
			fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult<Self> {
				iatoi!(iter)
			}

			fn atoi_iter_end<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult<Self> {
				iatoi!(iter, end)
			}
		}
	}
}


checked_impl!(u, u8);
checked_impl!(u, u16);
checked_impl!(u, u32);
checked_impl!(u, u64);
checked_impl!(u, u128);
checked_impl!(u, usize);

checked_impl!(i, i8);
checked_impl!(i, i16);
checked_impl!(i, i32);
checked_impl!(i, i64);
checked_impl!(i, i128);
checked_impl!(i, isize);



#[cfg(test)]
mod tests {
	use super::Atoi;
	use super::AtoiErr;
	use super::AtoiResult;
	
	#[test]
	fn overflow() {
		//max self + 1
		assert_eq!(u8::atoi(b"256"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u16::atoi(b"65536"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u32::atoi(b"4294967296"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u64::atoi(b"18446744073709551616"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u128::atoi(b"340282366920938463463374607431768211456"), Result::Err(AtoiErr::Overflow));
		
		//max self + 1
		assert_eq!(i8::atoi(b"128"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i16::atoi(b"32768"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i32::atoi(b"2147483648"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i64::atoi(b"9223372036854775808"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i64::atoi(b"170141183460469231731687303715884105728"), Result::Err(AtoiErr::Overflow));
	}
	
	#[test]
	fn empty_num() {
		assert_eq!(u8::atoi(b"0"), Result::Ok(0));
		assert_eq!(u16::atoi(b"0"), Result::Ok(0));
		assert_eq!(u32::atoi(b"0"), Result::Ok(0));
		assert_eq!(u64::atoi(b"0"), Result::Ok(0));
		assert_eq!(u128::atoi(b"0"), Result::Ok(0));
		
		assert_eq!(i8::atoi(b"0"), Result::Ok(0));
		assert_eq!(i16::atoi(b"0"), Result::Ok(0));
		assert_eq!(i32::atoi(b"0"), Result::Ok(0));
		assert_eq!(i64::atoi(b"0"), Result::Ok(0));
		assert_eq!(i128::atoi(b"0"), Result::Ok(0));
	}
	
	
	#[test]
	fn unsigned_value() {
		//unk unsigned num
		assert_eq!(u8::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u16::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u32::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u64::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u128::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		
		//min self
		assert_eq!(u8::atoi(b"0"), Result::Ok(0));
		assert_eq!(u16::atoi(b"0"), Result::Ok(0));
		assert_eq!(u32::atoi(b"0"), Result::Ok(0));
		assert_eq!(u64::atoi(b"0"), Result::Ok(0));
		assert_eq!(u128::atoi(b"0"), Result::Ok(0));
		
		//max self
		assert_eq!(u8::atoi(b"255"), Result::Ok(255));
		assert_eq!(u16::atoi(b"65535"), Result::Ok(65535));
		assert_eq!(u32::atoi(b"4294967295"), Result::Ok(4294967295));
		assert_eq!(u64::atoi(b"18446744073709551615"), Result::Ok(18446744073709551615));
		assert_eq!(u128::atoi(b"340282366920938463463374607431768211455"), Result::Ok(340282366920938463463374607431768211455));
	}
	
	#[test]
	fn signed_value() {
		//128
		assert_eq!(i8::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i16::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i32::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i64::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i128::atoi(b"-128"), Result::Ok(-128));
		
		//-1
		assert_eq!(i8::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i16::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i32::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i64::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i128::atoi(b"-1"), Result::Ok(-1));
		
				
		//max self
		assert_eq!(i8::atoi(b"127"), Result::Ok(127));
		assert_eq!(i16::atoi(b"32767"), Result::Ok(32767));
		assert_eq!(i32::atoi(b"2147483647"), Result::Ok(2147483647));
		assert_eq!(i64::atoi(b"9223372036854775807"), Result::Ok(9223372036854775807));
		assert_eq!(i128::atoi(b"170141183460469231731687303715884105727"), Result::Ok(170141183460469231731687303715884105727));
		
		//min self
		assert_eq!(i8::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i16::atoi(b"-32768"), Result::Ok(-32768));
		assert_eq!(i32::atoi(b"-2147483648"), Result::Ok(-2147483648));
		assert_eq!(i64::atoi(b"-9223372036854775808"), Result::Ok(-9223372036854775808));
		assert_eq!(i128::atoi(b"-170141183460469231731687303715884105728"), Result::Ok(-170141183460469231731687303715884105728));
		
	}
	
	#[test]
	fn unk_value() {
		assert_eq!(u8::atoi(b"128U"), Result::Err(AtoiErr::ByteUnk(b'U')));
		assert_eq!(u16::atoi(b"128L"), Result::Err(AtoiErr::ByteUnk(b'L')));
		assert_eq!(u32::atoi(b"128I"), Result::Err(AtoiErr::ByteUnk(b'I')));
		assert_eq!(u64::atoi(b"128N"), Result::Err(AtoiErr::ByteUnk(b'N')));
		assert_eq!(u128::atoi(b"128."), Result::Err(AtoiErr::ByteUnk(b'.')));
	}
	
	#[test]
	fn iter_end() {
		assert_eq!(u8::atoi_end(b"128U", b'U'), Result::Ok(128));
		assert_eq!(u16::atoi_end(b"128L", b'L'), Result::Ok(128));
		assert_eq!(u32::atoi_end(b"128I", b'I'), Result::Ok(128));
		assert_eq!(u64::atoi_end(b"128N", b'N'), Result::Ok(128));
		assert_eq!(u128::atoi_end(b"128.", b'.'), Result::Ok(128));
	}
}





