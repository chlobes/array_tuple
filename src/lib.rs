pub trait ArrayTuple {
	type Array;
	type Tuple;
	fn into_array(self) -> Self::Array;
	fn into_tuple(self) -> Self::Tuple;
}

/*impl<T> ArrayTuple for () {
	type Array = [T; 0];
	type Tuple = ();
	fn into_array(self) -> [T; 0] { [] }
	fn into_tuple(self) -> () { () }
}

impl<T> ArrayTuple for [T; 0] {
	type Array = Self;
	type Tuple = ();
	fn into_array(self) -> Self { [] }
	fn into_tuple(self) -> () { () }
}*/

use std::mem::transmute_copy;

impl<T> ArrayTuple for (T,) {
	type Array = [T; 1];
	type Tuple = Self;
	fn into_array(self) -> [T; 1] { unsafe{transmute_copy(&self)} }
	fn into_tuple(self) -> (T,) { self }
}

impl<T> ArrayTuple for [T; 1] {
	type Array = Self;
	type Tuple = (T,);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,) { unsafe{transmute_copy(&self)} }
}

impl<T> ArrayTuple for (T, T) {
	type Array = [T; 2];
	type Tuple = Self;
	fn into_array(self) -> [T; 2] { unsafe{transmute_copy(&self)} }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 2] {
	type Array = Self;
	type Tuple = (T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T) { unsafe{transmute_copy(&self)} }
}

impl<T> ArrayTuple for (T, T, T) {
	type Array = [T; 3];
	type Tuple = Self;
	fn into_array(self) -> [T; 3] { unsafe{transmute_copy(&self)} }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 3] {
	type Array = Self;
	type Tuple = (T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T) { unsafe{transmute_copy(&self)} }
}

impl<T> ArrayTuple for (T, T, T, T) {
	type Array = [T; 4];
	type Tuple = Self;
	fn into_array(self) -> [T; 4] { unsafe{transmute_copy(&self)} }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 4] {
	type Array = Self;
	type Tuple = (T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T) { unsafe{transmute_copy(&self)} }
}
