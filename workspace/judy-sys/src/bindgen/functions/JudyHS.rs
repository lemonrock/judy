// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


extern "C"
{
	pub fn JudyHSDel(arg1: PPvoid_t, arg2: *mut c_void, arg3: Word_t, PJError: PJError_t) -> c_int;
	pub fn JudyHSFreeArray(arg1: PPvoid_t, PJError: PJError_t) -> Word_t;
	pub fn JudyHSGet(arg1: Pcvoid_t, arg2: *mut c_void, arg3: Word_t) -> PPvoid_t;
	pub fn JudyHSIns(arg1: PPvoid_t, arg2: *mut c_void, arg3: Word_t, PJError: PJError_t) -> PPvoid_t;
}
