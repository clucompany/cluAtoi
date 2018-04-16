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

/*!
Parsing the byte sequence of the ascii characters and safely converting them to integers.


# Capabilities:
  1.  Convert ASCII byte sequences to integer primitives Rust.
  2.  Protection against overflow of numbers
  3.  Accounting for the features of signed, unsigned primitives
  4.  Return of the transfer in case of an error
  5.  Using IterByteArray

# Use:

```rust
use cluatoi::Atoi;

fn main() {
	let isize = isize::atoi(b"-1245").unwrap(); 		
	//-1245isize
	
	let usize = usize::atoi(b"1245").unwrap();
	//1245usize
	
	let my_int = u64::atoi_stop(b"1245T", b'T').unwrap(); 	
	//1245u64
	
	let my_int = u64::atoi_iter(b"1245".iter()).unwrap(); 	
	//1245u64
}
```

```rust
use cluatoi::Atoi;

fn main() {
	let array = b"1024!18446744073709551610!-1!X";
	let mut array_iter = array.iter();
	
	
	let num_0 = u64::atoi_iter_wait_stop(&mut array_iter, b'!').unwrap(); 
	//1024u64
	
	let num_1 = u64::atoi_iter_wait_stop(&mut array_iter, b'!').unwrap(); 
	//18446744073709551610u64
	
	let num_err = usize::atoi_iter_wait_stop(&mut array_iter, b'!');
	//ERROR, ISIZE VALUE + USIZE TYPE
	//Err(ByteUnk(45))
	
	let end_byte = array_iter.next().unwrap();
	//X
	
	println!("{}!{}!{:?}!{}", num_0, num_1, num_err, end_byte);
	//1024!18446744073709551610!Err(ByteUnk(45))!88
}
```

*/


///Type result Atoi.
pub type AtoiResult<T> = Result<T, AtoiErr>;

///Parsing the byte sequence of the ascii characters and safely converting them to integers.
pub trait Atoi<T> {
	///Array parsing.
	///
	///```rust
	///use cluatoi::Atoi;
	///
	///fn main() {
	///	if let Ok(num) = isize::atoi(b"-1024") {
	///		println!("TEST {}", num);
	///	}
	///}
	///```
	fn atoi<'a>(array: &'a [u8]) -> AtoiResult<T>;
	
	
	///Array parsing and stopping on the 'X' character.
	///
	///```rust
	///use cluatoi::Atoi;
	///
	///fn main() {
	///	let array = b"A1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	if let Ok(num) = isize::atoi_stop(&mut array_iter, b'~') {	
	///		//1024isize
	///
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	
	///	//0
	///}
	///```
	///
	///```rust
	///fn main() {
	///	let array = b"A1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	//USIZE!!!
	///	if let Ok(num) = usize::atoi_stop(&mut array_iter, b'~') {	
	///		// AtoiErr::UnkSymbol(b'-')
	///
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	//1
	///}
	///```
	///
	fn atoi_stop<'a>(array: &'a [u8], stop: u8) -> AtoiResult<T>;
	
	///An array analysis using an iterator and waiting for an "X" character even if an error occurred.
	///
	///```rust
	///fn main() {
	///	let array = b"A-1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	//USIZE!!!
	///	if let Ok(num) = usize::atoi_wait_stop(&mut array_iter, b'~') {	
	///		// AtoiErr::UnkSymbol(b'-')
	///
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	
	///	//->0	?NEXT -> END ARRAY.
	///}
	///```
	///
	fn atoi_wait_stop<'a>(array: &'a [u8], stop: u8) -> AtoiResult<T>;
	
	
	///Array parsing using an iterator.
	///
	///```rust
	///fn main() {
	///	let array = b"A1024";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	if let Ok(num) = isize::atoi_iter(&mut array_iter) {	
	///		//1024isize
	///
	///		println!("TEST {}", num);
	///	}
	///}
	///```
	fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult<T>;
	
	
	///Array parsing using an iterator and stopping on the 'X' character.
	///
	///```rust
	///fn main() {
	///	let array = b"A1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	if let Ok(num) = isize::atoi_iter_stop(&mut array_iter, b'~') {	
	///		//1024isize
	///		
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	//0
	///}
	///```
	///
	///```rust
	///fn main() {
	///	let array = b"A-1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	//USIZE!!!
	///	if let Ok(num) = usize::atoi_iter_stop(&mut array_iter, b'~') {	
	///		// AtoiErr::UnkSymbol(b'-')
	///
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	
	///	//->1	?NEXT	-> b"0245~0"
	///
	///}
	///```
	///
	fn atoi_iter_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult<T>;
	
	
	///An array analysis using an iterator and waiting for an "X" character even if an error occurred.
	///
	///```rust
	///fn main() {
	///	let array = b"A-1024~0";
	///
	///	let mut array_iter = array.iter();
	///	let _ignore = array_iter.next();	//A
	///
	///	//USIZE!!!
	///	if let Ok(num) = usize::atoi_iter_wait_stop(&mut array_iter, b'~') {	
	///		// AtoiErr::UnkSymbol(b'-')
	///
	///		println!("TEST {}", num);
	///	}
	///	
	///	let index =  array_iter.next();	
	///	//->0	?NEXT -> END ARRAY.
	///}
	///```
	///
	fn atoi_iter_wait_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult<T>;
}

///Result trait Atoi
#[derive(Debug, PartialEq, Clone)]
pub enum AtoiErr {
	///Overflow of number.
	///
	///```rust
	///assert_eq!(u8::atoi(b"256"), Result::Err(AtoiErr::Overflow));
	///assert_eq!(u16::atoi(b"65536"), Result::Err(AtoiErr::Overflow));
	///assert_eq!(u32::atoi(b"4294967296"), Result::Err(AtoiErr::Overflow));
	///assert_eq!(u64::atoi(b"18446744073709551616"), Result::Err(AtoiErr::Overflow));
	///```
	Overflow,
	
	///In the byte sequence, an unknown character was used.
	///
	///```rust
	///assert_eq!(u8::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
	///assert_eq!(u16::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
	///assert_eq!(u32::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
	///assert_eq!(u64::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
	///
	///assert_eq!(u8::atoi(b"128U"), Result::Err(AtoiErr::ByteUnk(b'U')));
	///assert_eq!(u16::atoi(b"128L"), Result::Err(AtoiErr::ByteUnk(b'L')));
	///assert_eq!(u32::atoi(b"128I"), Result::Err(AtoiErr::ByteUnk(b'I')));
	///assert_eq!(u64::atoi(b"128N"), Result::Err(AtoiErr::ByteUnk(b'N')));
	///```
	///
	ByteUnk(u8),
}


//ATOI BUILDER
macro_rules! atoi_build {
	
	//UNSIGNED ITERATION
	(unsigned, $iter:expr) => {{
		let mut result: Self = 0;
		while let Some(a) = $iter.next() {
			atoi_build_fn!(+, *a, result);
		}
		
		return Ok(result);
	}};
	
	//UNSIGNED ITERATION + STOP END CHAR
	(unsigned, $iter:expr, $end:expr) => {{
		let mut result: Self = 0;
		while let Some(a) = $iter.next() {
			atoi_build_fn!(+, *a, result, $end);
		}
		
		return Ok(result);
	}};
	
	//UNSIGNED ITERATION + WAIT END CHAR
	(unsigned_wait_end, $iter:expr, $end:expr) => {{
		let mut result: Self = 0;
		while let Some(a) = $iter.next() {
			atoi_build_fn!(+wait_end, *a, result, $iter, $end);
		}
		
		return Ok(result);
	}};
	
	//SIGNED ITERATION
	(signed, $iter:expr) => {{
		let mut result: Self = 0;
		if let Some(a) = $iter.next() {
			match *a {
				b'-' => {
					//NEGATIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(-, *a, result);
					}
				},
				b'+' => {
					//POSITIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+, *a, result);
					}
				},
				a => {
					atoi_build_fn!(+, a, result);
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+, *a, result);
					}
				}
			}
		}
		return Ok(result);
	}};
	
	//SIGNED ITERATION + STOP END CHAR
	(signed, $iter:expr, $end:expr) => {{
		let mut result: Self = 0;
		if let Some(a) = $iter.next() {
			match *a {
				b'-' => {
					//NEGATIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(-, *a, result, $end);
					}
				},
				b'+' => {
					//POSITIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+, *a, result, $end);
					}
				},
				a => {
					atoi_build_fn!(+, a, result, $end);
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+, *a, result, $end);
					}
				}
			}
		}
		return Ok(result);
	}};
	
	//SIGNED ITERATION + WAIT END CHAR
	(signed_wait_end, $iter:expr, $end:expr) => {{
		let mut result: Self = 0;
		if let Some(a) = $iter.next() {
			match *a {
				b'-' => {
					//NEGATIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(-wait_end, *a, result, $iter, $end);
					}
				},
				b'+' => {
					//POSITIVE
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+wait_end, *a, result, $iter, $end);
					}
				},
				a => {
					atoi_build_fn!(+wait_end, a, result, $iter, $end);
					while let Some(a) = $iter.next() {
						atoi_build_fn!(+wait_end, *a, result, $iter, $end);
					}
				}
			}
		}
		return Ok(result);
	}};
}

///Signed Atoi Macros
macro_rules! atoi_build_fn {
	//ADD FN
	(+, $a:expr, $result:expr) => {
		if !($a >= b'0' && $a <= b'9') {
			return Err(AtoiErr::ByteUnk($a));
		}
		
		match $result.checked_mul(10) {
			Some(s) => {
				match s.checked_add(($a - b'0') as Self) {
					Some(s) => {
						$result = s;
					},
					_ => {
						return Err(AtoiErr::Overflow);
					},
				}
			},
			_ => {
				return Err(AtoiErr::Overflow);
			},
		}
	};
	
	//ADD FN + END CHAR
	(+, $a:expr, $result:expr, $end:expr) => {
		if $end == $a {
			return Ok($result);
		}
		atoi_build_fn!(+, $a, $result);
	};
	
	//ADD FN + END CHAR + WHILE DO END POS
	(+wait_end, $a:expr, $result:expr, $iter:expr, $end:expr) => {
		if $end == $a {
			return Ok($result);
		}
		if !($a >= b'0' && $a <= b'9') {
			//WHAT?
			//
			//let array = b"-10a0";
			//let mut iter = array.iter();
			//
			//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
			//
			//let end_num = iter.next(); // Some(b'0')
			//
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::ByteUnk($a));
		}
		match $result.checked_mul(10) {
			Some(s) => {
				match s.checked_add(($a - b'0') as Self) {
					Some(s) => {
						$result = s;
						//STOP THIS.
					},
					_ => {
						//WHAT?
						//
						//let array = b"-10a0";
						//let mut iter = array.iter();
						//
						//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
						//
						//let end_num = iter.next(); // Some(b'0')
						//
						while let Some(a) = $iter.next() {
							if *a == $end {
								break;
							}
						}
						return Err(AtoiErr::Overflow);
					},
				}
			},
			_ => {
				//WHAT?
				//
				//let array = b"-10a0";
				//let mut iter = array.iter();
				//
				//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
				//
				//let end_num = iter.next(); // Some(b'0')
				//
				while let Some(a) = $iter.next() {
					if *a == $end {
						break;
					}
				}
				return Err(AtoiErr::Overflow);
			},
		}
	};
	
	
	//SUB FN
	(-, $a:expr, $result:expr) => {
		if !($a >= b'0' && $a <= b'9') {
			return Err(AtoiErr::ByteUnk($a));
		}
		
		match $result.checked_mul(10) {
			Some(s) => {
				match s.checked_sub(($a - b'0') as Self) {
					Some(s) => {
						$result = s;
					},
					_ => {
						return Err(AtoiErr::Overflow);
					},
				}
			},
			_ => {
				return Err(AtoiErr::Overflow);
			},
		}
	};
	
	//SUB FN + END CHAR
	(-, $a:expr, $result:expr, $end:expr) => {
		if $end == $a {
			return Ok($result);
		}
		atoi_build_fn!(-, $a, $result);
	};
	
	//SUB FN + END CHAR + WHILE DO END POS
	(-wait_end, $a:expr, $result:expr, $iter:expr, $end:expr) => {
		if $end == $a {
			return Ok($result);
		}
		if !($a >= b'0' && $a <= b'9') {
			//WHAT?
			//
			//let array = b"-10a0";
			//let mut iter = array.iter();
			//
			//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
			//
			//let end_num = iter.next(); // Some(b'0')
			//
			while let Some(a) = $iter.next() {
				if *a == $end {
					break;
				}
			}
			return Err(AtoiErr::ByteUnk($a));
		}
		match $result.checked_mul(10) {
			Some(s) => {
				match s.checked_sub(($a - b'0') as Self) {
					Some(s) => {
						$result = s;
						//STOP THIS.
					},
					_ => {
						//WHAT?
						//
						//let array = b"-10a0";
						//let mut iter = array.iter();
						//
						//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
						//
						//let end_num = iter.next(); // Some(b'0')
						//
						while let Some(a) = $iter.next() {
							if *a == $end {
								break;
							}
						}
						return Err(AtoiErr::Overflow);
					},
				}
			},
			_ => {
				//WHAT?
				//
				//let array = b"-10a0";
				//let mut iter = array.iter();
				//
				//let num = usize::iter_stop(&mut iter, b'a'); <- UNK BYTE b'-'
				//
				//let end_num = iter.next(); // Some(b'0')
				//
				while let Some(a) = $iter.next() {
					if *a == $end {
						break;
					}
				}
				return Err(AtoiErr::Overflow);
			},
		}
	};
	
}





macro_rules! atoi_build_type {
	(u, $t:ty) => {
		impl Atoi<Self> for $t {
			fn atoi<'a>(array: &'a [u8]) -> AtoiResult< Self > {				
				Self::atoi_iter(&mut array.iter())
			}

			fn atoi_stop<'a>(array: &'a [u8], end: u8) -> AtoiResult< Self > {				
				Self::atoi_iter_stop(&mut array.iter(), end)
			}
			fn atoi_wait_stop<'a>(array: &'a [u8], end: u8) -> AtoiResult< Self > {				
				Self::atoi_iter_wait_stop(&mut array.iter(), end)
			}
			
			fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult< Self > {
				atoi_build!(unsigned, iter);
			}

			fn atoi_iter_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult< Self > {
				atoi_build!(unsigned, iter, end);
			}
			fn atoi_iter_wait_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult< Self > {
				atoi_build!(unsigned_wait_end, iter, end);
			}
		}
	};
	(i, $t:ty) => {
		impl Atoi<Self> for $t {
			fn atoi<'a>(array: &'a [u8]) -> AtoiResult< Self > {				
				Self::atoi_iter(&mut array.iter())
			}

			fn atoi_stop<'a>(array: &'a [u8], end: u8) -> AtoiResult< Self > {				
				Self::atoi_iter_stop(&mut array.iter(), end)
			}
			fn atoi_wait_stop<'a>(array: &'a [u8], end: u8) -> AtoiResult< Self > {
				Self::atoi_iter_wait_stop(&mut array.iter(), end)
			}
			
			fn atoi_iter<'a>(iter: &'a mut Iterator<Item=&u8>) -> AtoiResult< Self > {
				atoi_build!(signed, iter);
			}

			fn atoi_iter_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult< Self > {
				atoi_build!(signed, iter, end);
			}
			fn atoi_iter_wait_stop<'a>(iter: &'a mut Iterator<Item=&u8>, end: u8) -> AtoiResult< Self > {
				atoi_build!(signed_wait_end, iter, end);
			}
		}
	}
}


atoi_build_type!(u, u8);
atoi_build_type!(u, u16);
atoi_build_type!(u, u32);
atoi_build_type!(u, u64);

atoi_build_type!(i, i8);
atoi_build_type!(i, i16);
atoi_build_type!(i, i32);
atoi_build_type!(i, i64);

atoi_build_type!(i, isize);
atoi_build_type!(u, usize);


#[cfg(unstable)]
atoi_build_type!(i, i128);
#[cfg(unstable)]
atoi_build_type!(u, u128);


#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn overflow() {
		//max self + 1
		assert_eq!(u8::atoi(b"256"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u16::atoi(b"65536"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u32::atoi(b"4294967296"), Result::Err(AtoiErr::Overflow));
		assert_eq!(u64::atoi(b"18446744073709551616"), Result::Err(AtoiErr::Overflow));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"340282366920938463463374607431768211456"), Result::Err(AtoiErr::Overflow));
		
		//max self + 1
		assert_eq!(i8::atoi(b"128"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i16::atoi(b"32768"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i32::atoi(b"2147483648"), Result::Err(AtoiErr::Overflow));
		assert_eq!(i64::atoi(b"9223372036854775808"), Result::Err(AtoiErr::Overflow));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"170141183460469231731687303715884105728"), Result::Err(AtoiErr::Overflow));
	}
	
	#[test]
	fn empty_num() {
		assert_eq!(u8::atoi(b"0"), Result::Ok(0));
		assert_eq!(u16::atoi(b"0"), Result::Ok(0));
		assert_eq!(u32::atoi(b"0"), Result::Ok(0));
		assert_eq!(u64::atoi(b"0"), Result::Ok(0));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"0"), Result::Ok(0));
		
		assert_eq!(i8::atoi(b"0"), Result::Ok(0));
		assert_eq!(i16::atoi(b"0"), Result::Ok(0));
		assert_eq!(i32::atoi(b"0"), Result::Ok(0));
		assert_eq!(i64::atoi(b"0"), Result::Ok(0));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"0"), Result::Ok(0));
	}
	
	
	#[test]
	fn unsigned_value() {
		//unk unsigned num
		assert_eq!(u8::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u16::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u32::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		assert_eq!(u64::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"-128"), Result::Err(AtoiErr::ByteUnk(b'-')));
		
		//min self
		assert_eq!(u8::atoi(b"0"), Result::Ok(0));
		assert_eq!(u16::atoi(b"0"), Result::Ok(0));
		assert_eq!(u32::atoi(b"0"), Result::Ok(0));
		assert_eq!(u64::atoi(b"0"), Result::Ok(0));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"0"), Result::Ok(0));
		
		//max self
		assert_eq!(u8::atoi(b"255"), Result::Ok(255));
		assert_eq!(u16::atoi(b"65535"), Result::Ok(65535));
		assert_eq!(u32::atoi(b"4294967295"), Result::Ok(4294967295));
		assert_eq!(u64::atoi(b"18446744073709551615"), Result::Ok(18446744073709551615));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"340282366920938463463374607431768211455"), Result::Ok(340282366920938463463374607431768211455));
	}
	
	#[test]
	fn signed_value() {
		//128
		assert_eq!(i8::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i16::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i32::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i64::atoi(b"-128"), Result::Ok(-128));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"-128"), Result::Ok(-128));
		
		//-1
		assert_eq!(i8::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i16::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i32::atoi(b"-1"), Result::Ok(-1));
		assert_eq!(i64::atoi(b"-1"), Result::Ok(-1));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"-1"), Result::Ok(-1));
		
				
		//max self
		assert_eq!(i8::atoi(b"127"), Result::Ok(127));
		assert_eq!(i16::atoi(b"32767"), Result::Ok(32767));
		assert_eq!(i32::atoi(b"2147483647"), Result::Ok(2147483647));
		assert_eq!(i64::atoi(b"9223372036854775807"), Result::Ok(9223372036854775807));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"170141183460469231731687303715884105727"), Result::Ok(170141183460469231731687303715884105727));
		
		//min self
		assert_eq!(i8::atoi(b"-128"), Result::Ok(-128));
		assert_eq!(i16::atoi(b"-32768"), Result::Ok(-32768));
		assert_eq!(i32::atoi(b"-2147483648"), Result::Ok(-2147483648));
		assert_eq!(i64::atoi(b"-9223372036854775808"), Result::Ok(-9223372036854775808));
		
		#[cfg(unstable)]
		assert_eq!(i128::atoi(b"-170141183460469231731687303715884105728"), Result::Ok(-170141183460469231731687303715884105728));
		
	}
	
	#[test]
	fn unk_value() {
		assert_eq!(u8::atoi(b"128U"), Result::Err(AtoiErr::ByteUnk(b'U')));
		assert_eq!(u16::atoi(b"128L"), Result::Err(AtoiErr::ByteUnk(b'L')));
		assert_eq!(u32::atoi(b"128I"), Result::Err(AtoiErr::ByteUnk(b'I')));
		assert_eq!(u64::atoi(b"128N"), Result::Err(AtoiErr::ByteUnk(b'N')));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi(b"128."), Result::Err(AtoiErr::ByteUnk(b'.')));
	}
	
	#[test]
	fn atoi_stop() {
		assert_eq!(u8::atoi_stop(b"128U", b'U'), Result::Ok(128));
		assert_eq!(u16::atoi_stop(b"128L", b'L'), Result::Ok(128));
		assert_eq!(u32::atoi_stop(b"128I", b'I'), Result::Ok(128));
		assert_eq!(u64::atoi_stop(b"128N", b'N'), Result::Ok(128));
		
		#[cfg(unstable)]
		assert_eq!(u128::atoi_stop(b"128.", b'.'), Result::Ok(128));
	}
	
	#[test]
	fn atoi_iter_stop() {
		let array = b"-100!E";
		let stop = b'!';
		
		{//STOP ON ERROR
			let mut iter = array.iter();
			
			assert_eq!(u64::atoi_iter_stop(&mut iter, stop), Result::Err(AtoiErr::ByteUnk(b'-')));
			
			
			assert_eq!(iter.next(), Some(&b'1'));
		}
		
		{//CONTINUE ON ERROR, AND WAIT END CHAR
			let mut iter = array.iter();
			
			assert_eq!(u64::atoi_iter_wait_stop(&mut iter, stop), Result::Err(AtoiErr::ByteUnk(b'-')));
			
			
			assert_eq!(iter.next(), Some(&b'E'));
		}
	}
}





