pub trait ArrayTuple: Sized {
	type Array: ArrayTuple<Tuple=Self::Tuple>;
	type Tuple;
	fn into_array(self) -> Self::Array;
	fn into_tuple(self) -> Self::Tuple {
		self.into_array().into_tuple()
	}
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

impl<T> ArrayTuple for (T,T) {
	type Array = [T; 2];
	type Tuple = Self;
	fn into_array(self) -> [T; 2] { let (x1,x2) = self; [x1,x2] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 2] {
	type Array = Self;
	type Tuple = (T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T) { let [x1,x2] = self; (x1,x2) }
}

impl<T> ArrayTuple for (T,T,T) {
	type Array = [T; 3];
	type Tuple = Self;
	fn into_array(self) -> [T; 3] { let (x1,x2,x3) = self; [x1,x2,x3] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 3] {
	type Array = Self;
	type Tuple = (T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T) { let [x1,x2,x3] = self; (x1,x2,x3) }
}

impl<T> ArrayTuple for (T,T,T,T) {
	type Array = [T; 4];
	type Tuple = Self;
	fn into_array(self) -> [T; 4] { let (x1,x2,x3,x4) = self; [x1,x2,x3,x4] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 4] {
	type Array = Self;
	type Tuple = (T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T) { let [x1,x2,x3,x4] = self; (x1,x2,x3,x4) }
}

impl<T> ArrayTuple for (T,T,T,T,T) {
	type Array = [T; 5];
	type Tuple = Self;
	fn into_array(self) -> [T; 5] { let (x1,x2,x3,x4,x5) = self; [x1,x2,x3,x4,x5] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 5] {
	type Array = Self;
	type Tuple = (T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T) { let [x1,x2,x3,x4,x5] = self; (x1,x2,x3,x4,x5) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T) {
	type Array = [T; 6];
	type Tuple = Self;
	fn into_array(self) -> [T; 6] { let (x1,x2,x3,x4,x5,x6) = self; [x1,x2,x3,x4,x5,x6] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 6] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6] = self; (x1,x2,x3,x4,x5,x6) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T) {
	type Array = [T; 7];
	type Tuple = Self;
	fn into_array(self) -> [T; 7] { let (x1,x2,x3,x4,x5,x6,x7) = self; [x1,x2,x3,x4,x5,x6,x7] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 7] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7] = self; (x1,x2,x3,x4,x5,x6,x7) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T) {
	type Array = [T; 8];
	type Tuple = Self;
	fn into_array(self) -> [T; 8] { let (x1,x2,x3,x4,x5,x6,x7,x8) = self; [x1,x2,x3,x4,x5,x6,x7,x8] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 8] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8] = self; (x1,x2,x3,x4,x5,x6,x7,x8) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T) {
	type Array = [T; 9];
	type Tuple = Self;
	fn into_array(self) -> [T; 9] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 9] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 10];
	type Tuple = Self;
	fn into_array(self) -> [T; 10] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 10] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 11];
	type Tuple = Self;
	fn into_array(self) -> [T; 11] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 11] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 12];
	type Tuple = Self;
	fn into_array(self) -> [T; 12] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 12] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 13];
	type Tuple = Self;
	fn into_array(self) -> [T; 13] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 13] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 14];
	type Tuple = Self;
	fn into_array(self) -> [T; 14] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 14] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 15];
	type Tuple = Self;
	fn into_array(self) -> [T; 15] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 15] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 16];
	type Tuple = Self;
	fn into_array(self) -> [T; 16] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 16] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 17];
	type Tuple = Self;
	fn into_array(self) -> [T; 17] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 17] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 18];
	type Tuple = Self;
	fn into_array(self) -> [T; 18] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 18] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 19];
	type Tuple = Self;
	fn into_array(self) -> [T; 19] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 19] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 20];
	type Tuple = Self;
	fn into_array(self) -> [T; 20] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 20] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 21];
	type Tuple = Self;
	fn into_array(self) -> [T; 21] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 21] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 22];
	type Tuple = Self;
	fn into_array(self) -> [T; 22] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 22] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 23];
	type Tuple = Self;
	fn into_array(self) -> [T; 23] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 23] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 24];
	type Tuple = Self;
	fn into_array(self) -> [T; 24] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 24] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 25];
	type Tuple = Self;
	fn into_array(self) -> [T; 25] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 25] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 26];
	type Tuple = Self;
	fn into_array(self) -> [T; 26] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 26] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 27];
	type Tuple = Self;
	fn into_array(self) -> [T; 27] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 27] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 28];
	type Tuple = Self;
	fn into_array(self) -> [T; 28] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 28] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 29];
	type Tuple = Self;
	fn into_array(self) -> [T; 29] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 29] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 30];
	type Tuple = Self;
	fn into_array(self) -> [T; 30] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 30] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 31];
	type Tuple = Self;
	fn into_array(self) -> [T; 31] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 31] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31) }
}

impl<T> ArrayTuple for (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) {
	type Array = [T; 32];
	type Tuple = Self;
	fn into_array(self) -> [T; 32] { let (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32) = self; [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32] }
	fn into_tuple(self) -> Self { self }
}

impl<T> ArrayTuple for [T; 32] {
	type Array = Self;
	type Tuple = (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T);
	fn into_array(self) -> Self { self }
	fn into_tuple(self) -> (T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T,T) { let [x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32] = self; (x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16,x17,x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,x31,x32) }
}
