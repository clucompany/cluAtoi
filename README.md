# Atoi 

[![Build Status](https://travis-ci.org/clucompany/cluAtoi.svg?branch=master)](https://travis-ci.org/clucompany/cluAtoi)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluatoi)](https://crates.io/crates/cluatoi)
[![Documentation](https://docs.rs/cluatoi/badge.svg)](https://docs.rs/cluatoi)

Parsing the byte sequence of the ascii characters and safely converting them to integers.

Capabilities:
1. Convert ASCII byte sequences to integer primitives Rust.
2. Protection against overflow of numbers
3. Accounting for the features of signed, unsigned primitives
4. Return of the transfer in case of an error
5. Using IterByteArray
	
Use:

#1

	extern crate cluatoi;
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

#2

	extern crate cluatoi;
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


Russian:

Разбор байтовой последовательности символов ascii и безопасное преобразование их в целые числа.

Открытая библиотека написанная на языке системного программирования Rust, используется для анализа и приведения последовательности байтов ascii во многие целочисленные примитивы языка Rust(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128). Присутстует возможность подставки byte итераторов для продолжения парсинга byte массивов без создания отдельного byte массива или byte среза.

Возможности:
1. Преобразование ASCII byte последовательности в целочисленные примитивы Rust.
2. Защита предотвращяющяя переполнения чисел
3. Учет особенностей signed, unsigned примитивов
4. Возврат перечисления в случае ошибки
5. Использование IterByteArray


License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
