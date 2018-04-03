# Atoi 

[![Build Status](https://travis-ci.org/clucompany/cluAtoi.svg?branch=master)](https://travis-ci.org/clucompany/cluAtoi)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE.txt)
[![crates.io](http://meritbadge.herokuapp.com/cluatoi)](https://crates.io/crates/cluatoi)
[![Documentation](https://docs.rs/cluatoi/badge.svg)](https://docs.rs/cluatoi)

Parsing the byte sequence of the ascii characters and safely converting them to integers.

An open library written in the language of system programming Rust is used to analyze and bring the sequence of ascii bytes to many integer primitives of the language Rust (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128). There is the possibility of using the byte iterators to continue parsing byte arrays without creating a separate byte array or byte slice.

Capabilities:
1. Convert ASCII byte sequences to integer primitives Rust.
2. Protection against overflow of numbers
3. Accounting for the features of signed, unsigned primitives
4. Return of the transfer in case of an error
5. Using IterByteArray
	
Use:

	extern crate cluatoi;
	use cluatoi::Atoi;

	fn main() {

		let array = b"-1245";
		let isize = isize::atoi(array).unwrap(); //  -1245isize

		let usize = usize::atoi(array).unwrap();  //  AtoiErr(ByteUnk(b'-'))


		let array_end = b"1245T";

		let my_int = u64::atoi_end(array_end, b'T').unwrap(); //1245u64

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
