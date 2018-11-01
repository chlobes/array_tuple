pub trait ArrayTuple {
	type Array;
	type Tuple;
	fn into_array(self) -> Self::Array;
	fn into_tuple(self) -> Self::Tuple;
}

impl<T: ArrayTuple> ArrayTuple for Option<T> {
	type Array = Option<<T as ArrayTuple>::Array>;
	type Tuple = Option<<T as ArrayTuple>::Tuple>;
	fn into_array(self) -> Self::Array { self.map(|t| t.into_array()) }
	fn into_tuple(self) -> Self::Tuple { self.map(|t| t.into_tuple()) }
}

/*impl<T> ArrayTuple for () {
	type Array = [T; 0];
	type Tuple = ();
	fn into_array(self) -> [T; 0] { [] }
	fn into_tuple(self) -> () { () }
}*/

impl<T> ArrayTuple for [T; 0] {
	type Array = Self;
	type Tuple = ();
	fn into_array(self) -> Self { [] }
	fn into_tuple(self) -> () { () }
}

/*macro_rules! impl_array_tuple {
	$($($N: num),+) => {$(
		impl<T> ArrayTuple for ($(T,$)+) { //$N times
			type Array = [T; $N];
			type Tuple = Self;
			fn into_array(self) -> Self::Array { let ($(x$N,$)+)=self; [$(x$N,$)+] }
			fn into_tuple(self) -> Self { self }
		}
		impl<T> ArrayTuple for [T; $N] {
			type Array = Self;
			type Tuple = ($(T,$)+); //$N times
			fn into_array(self) -> Self { self }
			fn into_tuple(self) -> Self::Tuple { let [$(x$N,$)+,]=self; ($(x$N,$)+) }
		}
	$)+}
}

impl_array_tuple(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32);*/

use std::mem::transmute_copy;

impl<T> ArrayTuple for (T,) {
	type Array = [T; 1];
	type Tuple = Self;
	fn into_array(self) -> Self::Array { let (x1,) = self; [x1] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 1] {
	type Array = Self;
	type Tuple = (T,);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> Self::Tuple { let [x1] = self; (x1,) }
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
