
// #Ulin Project 1718

use std::slice::Iter;

/// Result trait Atoi
pub type AtoiResult<T> = Result<T, AtoiErr>;

///Parsing the byte sequence of the ascii characters and safely converting them to integers.
pub trait Atoi<T> {

	///Array parsing
    fn atoi<'a>(array: &'a [u8]) -> AtoiResult<T>;
	///Array parsing and stopping on the 'X' character
    fn atoi_end<'a>(array: &'a [u8], end: u8) -> AtoiResult<T>;
	
	///Array parsing an array using an iterator
    fn atoi_iter<'a>(iter: &'a mut Iter<u8>) -> AtoiResult<T>;
	///Array parsing an array using an iterator and stopping on the 'X' character
    fn atoi_iter_end<'a>(iter: &'a mut Iter<u8>, end: u8) -> AtoiResult<T>;
}

#[derive(Debug)]
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

			fn atoi_iter<'a>(iter: &'a mut Iter<u8>) -> AtoiResult<Self> {
				atoi!(add, iter)
			}

			fn atoi_iter_end<'a>(iter: &'a mut Iter<u8>, end: u8) -> AtoiResult<Self> {
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
			
			fn atoi_iter<'a>(iter: &'a mut Iter<u8>) -> AtoiResult<Self> {
				iatoi!(iter)
			}

			fn atoi_iter_end<'a>(iter: &'a mut Iter<u8>, end: u8) -> AtoiResult<Self> {
				iatoi!(iter, end)
			}
		}
	}
}


checked_impl!(u, u8);
checked_impl!(u, u16);
checked_impl!(u, u32);
checked_impl!(u, u64);
checked_impl!(u, usize);

checked_impl!(i, i8);
checked_impl!(i, i16);
checked_impl!(i, i32);
checked_impl!(i, i64);
checked_impl!(i, isize);
