// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#[derive(Debug)]
pub struct JudyBoxMapValuesMutIterator<'a, T>
where T: 'a
{
	judyBoxMap: &'a mut JudyBoxMap<T>,
	index: c_ulong,
	pointerToValue: *mut *mut c_void,
}

impl<'a, T> Iterator for JudyBoxMapValuesMutIterator<'a, T>
where T: 'a
{
	type Item = &'a mut T;
	
	#[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
	{
		if self.pointerToValue.is_null()
		{
			return None;
		}
		
		let mutableReference = unsafe { *self.pointerToValue } as *mut T;
		self.pointerToValue = unsafe { JudyLNext((self.judyBoxMap.0).0, &mut self.index, null_mut()) };
		Some(unsafe { &mut *mutableReference })
	}
}

impl<'a, T> JudyBoxMapValuesMutIterator<'a, T>
where T: 'a
{
	#[inline(always)]
	fn new(judyBoxMap: &'a mut JudyBoxMap<T>) -> Self
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
