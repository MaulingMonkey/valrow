use valrow::*;
use abistr::{cstr, CStrPtr, CStrNonNull};
use core::cell::*;
use core::mem::size_of;
use core::ptr::NonNull;



// Example usage of Wrapper

fn main() {
    let mut channels = CHANNELS.lock().unwrap();
    channels.add(cstr!("#gamedev"));
    channels.add(cstr!("#rust"));

    let mut users = USERS.lock().unwrap();
    users.add(cstr!("MaulingMonkey"));

    // allows use of `channels` within ZST-requiring `users.for_each`
    let mut channels_mut = ValrowMut::new(&mut *channels);
    users.for_each(move |user| {
        let user = user.to_string_lossy();
        print!("PRIVMSG {user} :Check out the following channels!");

        channels_mut.for_each(move |channel| {
            let channel = channel.to_string_lossy();
            print!(" {channel}");

            #[cfg(xxx)] // won't compile: accessing `user` would make this closure a !ZST
            println!("PRIVMSG {user} :{channel}");
            // we could use a `thread_local! { static CURRENT_CHANNEL : ... = ...; }` instead?
        });
        println!();
    });

    // `channels_mut` borrow has ended, `channels` is once more accessible
    channels.for_each(move |channel| print!(" {}", channel.to_string_lossy()));
}



// Implementation of Wrapper

use sealed::*;
mod sealed {
    // make some single-instance ZSTs with an inaccessible `.0` field to act as marker types
    use std::sync::*;
    #[doc = "C++'s global, channels"] pub struct Channels(());
    #[doc = "C++'s global, users"   ] pub struct Users(());
    pub static CHANNELS : Mutex<Channels> = Mutex::new(Channels(()) );
    pub static USERS    : Mutex<Users   > = Mutex::new(Users(())    );
    unsafe impl valrow::BorrowableByValue for Channels  { type Abi = (); }
    unsafe impl valrow::BorrowableByValue for Users     { type Abi = (); }
}

impl Channels {
    pub fn add(&mut self, channel: CStrNonNull) {
        extern "C" { fn add_channel(channel: CStrNonNull); }
        unsafe { add_channel(channel) }
    }

    pub fn for_each<PerChannel: FnMut(CStrPtr)>(&self, mut per_channel: PerChannel) {
        extern "C" { fn for_each_channel(per_channel: extern "C" fn(channel: CStrPtr)); }
        unsafe { for_each_channel(adapt::<PerChannel>) }
        let _ = per_channel;

        let _ = StaticAssert::<PerChannel>::IS_ZST;
        extern "C" fn adapt<PerChannel: FnMut(CStrPtr)>(channel: CStrPtr) {
            (unsafe { NonNull::<PerChannel>::dangling().as_mut() })(channel)
        }
    }
}

impl Users {
    pub fn add(&mut self, user: CStrNonNull) {
        extern "C" { fn add_user(user: CStrNonNull); }
        unsafe { add_user(user) }
    }

    pub fn for_each<PerUser: FnMut(CStrPtr)>(&self, mut per_user: PerUser) {
        extern "C" { fn for_each_user(per_user: extern "C" fn(user: CStrPtr)); }
        unsafe { for_each_user(adapt::<PerUser>) };
        let _ = per_user;

        let _ = StaticAssert::<PerUser>::IS_ZST;
        extern "C" fn adapt<PerUser: FnMut(CStrPtr)>(user: CStrPtr) {
            (unsafe { NonNull::<PerUser>::dangling().as_mut() })(user)
        }
    }
}

/// ### Safety
/// Do not call this unless a ZST exists that you should have exclusive access to.
/// Note that the reference lifetime is unfortunately unbounded.
unsafe fn zst_mut<'zst, ZST>() -> &'zst mut ZST {
    let _ = StaticAssert::<ZST>::IS_ZST;
    unsafe { NonNull::<ZST>::dangling().as_mut() }
}

struct StaticAssert<T>(T);
impl<T> StaticAssert<T> {
    const IS_ZST : () = assert!(0 == size_of::<T>(), "T is not a ZST");
}
