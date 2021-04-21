use crate::pool::connection::Idle;
use crate::pool::shared::SharedPool;
use crate::pool::wait_list::WaitList;
use crate::{Connection, DefaultRuntime, Runtime};
use crossbeam_queue::ArrayQueue;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

mod connection;
mod options;
mod shared;
mod wait_list;

pub struct Pool<Rt: Runtime, C: Connection<Rt>> {
    shared: Arc<SharedPool<Rt, C>>,
}
