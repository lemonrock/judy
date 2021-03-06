// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[derive(Debug)]
pub struct JudyBoxMapIterator<'a, T>
where T: 'a
{
	judyBoxMap: &'a JudyBoxMap<T>,
	index: c_ulong,
	pointerToValue: *mut *mut c_void,
}

impl<'a, T> Iterator for JudyBoxMapIterator<'a, T>
where T: 'a
{
	type Item = (c_ulong, &'a T);
	
	#[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
	{
		if self.pointerToValue.is_null()
		{
			return None;
		}
		
		let mutableReference = unsafe { *self.pointerToValue } as *mut T;
		let index = self.index;
		self.pointerToValue = unsafe { JudyLNext((self.judyBoxMap.0).0, &mut self.index, null_mut()) };
		Some((index, unsafe { &*mutableReference }))
	}
}

impl<'a, T> JudyBoxMapIterator<'a, T>
where T: 'a
{
	#[inline(always)]
	fn new(judyBoxMap: &'a JudyBoxMap<T>) -> Self
	{
		let mut index = 0;
		let pointerToValue = unsafe { JudyLFirst((judyBoxMap.0).0, &mut index, null_mut()) };
		
		Self
		{
			judyBoxMap,
			index,
			pointerToValue,
		}
	}
}
