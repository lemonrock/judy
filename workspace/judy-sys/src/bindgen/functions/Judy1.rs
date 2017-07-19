// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


extern "C"
{
	pub fn Judy1ByCount(PArray: Pcvoid_t, Count: Word_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1Count(PArray: Pcvoid_t, Index1: Word_t, Index2: Word_t, PJError: PJError_t) -> Word_t;
	pub fn Judy1First(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1FirstEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1FreeArray(PPArray: PPvoid_t, PJError: PJError_t) -> Word_t;
	pub fn Judy1Last(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1LastEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1MemActive(PArray: Pcvoid_t) -> Word_t;
	pub fn Judy1MemUsed(PArray: Pcvoid_t) -> Word_t;
	pub fn Judy1Next(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1NextEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1Prev(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1PrevEmpty(PArray: Pcvoid_t, PIndex: *mut Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1Set(PPArray: PPvoid_t, Index: Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1SetArray(PPArray: PPvoid_t, Count: Word_t, PIndex: *const Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1Test(PArray: Pcvoid_t, Index: Word_t, PJError: PJError_t) -> c_int;
	pub fn Judy1Unset(PPArray: PPvoid_t, Index: Word_t, PJError: PJError_t) -> c_int;
}
