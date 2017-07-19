// This file is part of judy. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT. No part of judy-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of judy. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/judy/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)] 
#![feature(associated_consts)]


extern crate judy_sys;
extern crate libc;


use ::judy_sys::*;
use ::libc::c_long;
use ::libc::c_ulong;
use ::libc::c_void;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;


include!("Judy1.rs");
include!("JudyBoxMap.rs");
include!("JudyBoxMapIterator.rs");
include!("JudyBoxMapMutIterator.rs");
include!("JudyBoxMapValuesIterator.rs");
include!("JudyBoxMapValuesMutIterator.rs");
include!("JudyL.rs");
