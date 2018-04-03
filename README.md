# Atoi

Info:
An open library written in the language of system programming Rust is used to analyze and bring the sequence of ascii bytes to many integer primitives of the language Rust (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128). There is the possibility of using the byte iterators to continue parsing byte arrays without creating a separate byte array or byte slice.

Capabilities:
1. Convert ASCII byte sequences to integer primitives Rust.
2. Protection against overflow of numbers
3. Accounting for the features of signed, unsigned primitives
4. Return of the transfer in case of an error
5. Using IterByteArray

Информация:
Открытая библиотека написанная на языке системного программирования Rust, используется для анализа и приведения последовательности байтов ascii во многие целочисленные примитивы языка Rust(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128). Присутстует возможность подставки byte итераторов для продолжения парсинга byte массивов без создания отдельного byte массива или byte среза.

Возможности:
1. Преобразование ASCII byte последовательности в целочисленные примитивы Rust.
2. Защита предотвращяющяя переполнения чисел
3. Учет особенностей signed, unsigned примитивов
4. Возврат перечисления в случае ошибки
5. Использование IterByteArray

Use - Использование:
extern crate cluatoi;
use self::cluatoi::atoi;

fn main() {
  let array = b"-1245";
  let isize = isize::atoi(&array).unwrap(); //  -1245isize
  let usize = usize::atoi(&array).unwrap()  //  AtoiErr(ByteUnk(b'-'))
  
  let array_end = b"1245T";
  let my_int = u64::atoi(&array_end, b'T').unwrap(); //1245u64
}


#cluOpenLibs
#UlinProject 17-18
