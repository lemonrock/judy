// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


extern "C"
{
	pub fn JudySLDel(arg1: PPvoid_t, Index: *const u8, PJError: PJError_t) -> c_int;
	pub fn JudySLFirst(arg1: Pcvoid_t, Index: *mut u8, PJError: PJError_t) -> PPvoid_t;
	pub fn JudySLFreeArray(arg1: PPvoid_t, PJError: PJError_t) -> Word_t;
	pub fn JudySLGet(arg1: Pcvoid_t, Index: *const u8, PJError: PJError_t) -> PPvoid_t;
	pub fn JudySLIns(arg1: PPvoid_t, Index: *const u8, PJError: PJError_t) -> PPvoid_t;
	pub fn JudySLLast(arg1: Pcvoid_t, Index: *mut u8, PJError: PJError_t) -> PPvoid_t;
	pub fn JudySLNext(arg1: Pcvoid_t, Index: *mut u8, PJError: PJError_t) -> PPvoid_t;
	pub fn JudySLPrev(arg1: Pcvoid_t, Index: *mut u8, PJError: PJError_t) -> PPvoid_t;
}
