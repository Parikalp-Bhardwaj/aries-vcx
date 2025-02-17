#![allow(clippy::not_unsafe_ptr_arg_deref)]
#[macro_use]
extern crate serde_json;

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{
        $val
    }};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! secret {
    ($val:expr) => {{
        "_"
    }};
}

#[macro_use]
pub mod crypto;
pub mod environment;
pub mod sequence;
#[macro_use]
#[allow(unused_macros)]
pub mod test;
pub mod wql;

pub(crate) use indy_api_types::ErrorCode;

use indy_api_types::{CommandHandle, PoolHandle, SearchHandle, VdrHandle, WalletHandle};

pub fn next_wallet_handle() -> WalletHandle {
    WalletHandle(sequence::get_next_id())
}

pub fn next_pool_handle() -> PoolHandle {
    sequence::get_next_id()
}

pub fn next_command_handle() -> CommandHandle {
    sequence::get_next_id()
}

pub fn next_search_handle() -> SearchHandle {
    SearchHandle(sequence::get_next_id())
}

pub fn next_vdr_handle() -> VdrHandle {
    sequence::get_next_id()
}
