use crate::{ffi::*, println_debug};
use std::{
    ffi::c_int,
    marker::PhantomData,
    mem::size_of,
    sync::atomic::{AtomicUsize, Ordering},
};

/// only work with C pointer where address of a struct is the address of its first member
#[inline(always)]
pub(crate) unsafe fn from_ptr<'a, TFrom, TTo>(ptr: *mut TFrom) -> &'a mut TTo {
    &mut *(ptr as *mut _)
}
/// only work with C pointer where address of a struct is the address of its first member
/// can not be later used Box::from_raw
#[inline(always)]
pub(crate) fn to_ptr<TFrom, TTo>(v: TFrom) -> *mut TTo {
    &v as *const TFrom as _
}
/// only work with C pointer where address of a struct is the address of its first member
/// use Box::into_raw then later we can use Box::from_raw
#[inline(always)]
pub(crate) fn to_box_ptr<TFrom, TTo>(v: TFrom) -> *mut TTo {
    Box::into_raw(Box::new(v)) as _
}

pub(crate) trait PhantomWrapper<T> {
    // phantom trait to store cef typing
}

/// https://bitbucket.org/chromiumembedded/cef/wiki/UsingTheCAPI.md
#[repr(C)]
pub(crate) struct Brc<T: PhantomWrapper<C>, C> {
    // first member has the same C pointer with this wrapper struct
    v: T,
    count: AtomicUsize,
    phantom: PhantomData<C>,
    #[cfg(feature = "debug")]
    name: String,
}

impl<T: PhantomWrapper<C>, C> Brc<T, C> {
    unsafe extern "C" fn add_ref(ptr: *mut cef_base_ref_counted_t) {
        let this: &mut Self = from_ptr(ptr);
        println_debug!("add_ref {}", this.name);
        this.count.fetch_add(1, Ordering::Relaxed);
    }
    unsafe extern "C" fn release(ptr: *mut cef_base_ref_counted_t) -> c_int {
        let this: &mut Self = from_ptr(ptr);
        println_debug!("release {}", this.name);
        if this.count.fetch_sub(1, Ordering::Release) == 1 {
            drop(Box::from_raw(this));
            1
        } else {
            0
        }
    }
    unsafe extern "C" fn has_one_ref(ptr: *mut cef_base_ref_counted_t) -> c_int {
        let this: &mut Self = from_ptr(ptr);
        println_debug!("has_one_ref {}", this.name);
        if this.count.load(Ordering::SeqCst) >= 1 {
            1
        } else {
            0
        }
    }
    unsafe extern "C" fn has_at_least_one_ref(ptr: *mut cef_base_ref_counted_t) -> c_int {
        let this: &mut Self = from_ptr(ptr);
        println_debug!("has_at_least_one_ref {}", this.name);
        if this.count.load(Ordering::SeqCst) >= 1 {
            1
        } else {
            0
        }
    }
    pub(crate) fn new<F>(wrapper: F, #[cfg(feature = "debug")] name: String) -> *mut C
    where
        F: FnOnce(cef_base_ref_counted_t) -> T,
    {
        to_box_ptr(Self {
            v: wrapper(cef_base_ref_counted_t {
                size: size_of::<T>(),
                add_ref: Some(Self::add_ref),
                release: Some(Self::release),
                has_one_ref: Some(Self::has_one_ref),
                has_at_least_one_ref: Some(Self::has_at_least_one_ref),
            }),
            count: AtomicUsize::new(1),
            phantom: PhantomData,
            #[cfg(feature = "debug")]
            name,
        })
    }
}
