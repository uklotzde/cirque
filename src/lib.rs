// SPDX-FileCopyrightText: The cirque authors
// SPDX-License-Identifier: MPL-2.0

#![allow(rustdoc::invalid_rust_codeblocks)]
#![doc = include_str!("../README.md")]
#![warn(rust_2018_idioms)]
#![warn(rust_2021_compatibility)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(rustdoc::broken_intra_doc_links)]

use std::ops::{Deref, DerefMut};

use llq::{Node, Queue};

/// Non-realtime recycler
///
/// Receives items returned by the [`Consumer`] and recycles them before
/// reuse. Recycling might involve releasing resources outside of the
/// realtime context, e.g. dropping a buffer.
///
/// Items might be dropped at any time without recycling them. Don't rely
/// on that the method in this trait will be called for every consumed item!
pub trait Recycler<T> {
    /// Recycle an item
    fn recycle(&mut self, item: &mut T);
}

/// Create a new producer/consumer circular queue.
#[must_use]
pub fn new_producer_consumer<T, R>(
    recycler: R,
    recycling_capacity: usize,
) -> (Producer<T, R>, Consumer<T>) {
    let (producer_tx, consumer_rx) = Queue::new().split();
    let (consumer_tx, producer_rx) = Queue::new().split();
    let producer = Producer {
        tx: producer_tx,
        rx: producer_rx,
        recycler,
        recycling_capacity,
        recycled_nodes: Vec::with_capacity(recycling_capacity),
    };
    let consumer = Consumer {
        rx: consumer_rx,
        tx: consumer_tx,
    };
    (producer, consumer)
}

/// Non-realtime producer
#[allow(missing_debug_implementations)]
pub struct Producer<T, R> {
    tx: llq::Producer<T>,
    rx: llq::Consumer<T>,
    recycler: R,
    recycling_capacity: usize,
    recycled_nodes: Vec<llq::Node<T>>,
}

impl<T, R> Producer<T, R>
where
    R: Recycler<T>,
{
    /// Push a new item into the queue
    pub fn push(&mut self, item: T) {
        let node = self.new_node(item);
        self.tx.push(node);
    }

    fn new_node(&mut self, item: T) -> llq::Node<T> {
        // Reuse the recycled nodes first, because this does not involve any memory barriers.
        if let Some(mut node) = self.recycled_nodes.pop().or_else(|| self.rx.pop()) {
            let _ = std::mem::replace(&mut *node, item);
            return node;
        }
        // Allocate a new node
        Node::new(item)
    }

    /// Tune the recycling capacity
    ///
    /// The internal buffer will never shrink when lowering the capacity.
    pub fn tune_recycling_capacity(&mut self, recycling_capacity: usize) {
        if recycling_capacity < self.recycled_nodes.len() {
            self.recycled_nodes.truncate(recycling_capacity);
        } else {
            self.recycled_nodes
                .reserve(recycling_capacity - self.recycled_nodes.len());
        }
        self.recycling_capacity = recycling_capacity;
        debug_assert!(self.recycled_nodes.len() <= self.recycling_capacity);
        debug_assert!(self.recycled_nodes.capacity() >= self.recycling_capacity);
    }

    /// Drain all consumed items and recycle as much as possible
    ///
    /// Should be invoked periodically when not pushing new items.
    pub fn drain_and_recycle(&mut self) {
        while let Some(mut node) = self.rx.pop() {
            if self.recycled_nodes.len() < self.recycling_capacity {
                self.recycler.recycle(&mut *node);
                self.recycled_nodes.push(node);
            }
        }
    }
}

/// Consumable item
///
/// Should be handed back to the [`Consumer`] for recycling,
/// i.e. to keep it circling and avoid (de-)allocations.
#[allow(missing_debug_implementations)]
pub struct ConsumableItem<T>(llq::Node<T>);

impl<T> AsRef<T> for ConsumableItem<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for ConsumableItem<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> Deref for ConsumableItem<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for ConsumableItem<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

/// Realtime consumer
#[allow(missing_debug_implementations)]
pub struct Consumer<T> {
    rx: llq::Consumer<T>,
    tx: llq::Producer<T>,
}

impl<T> Consumer<T> {
    /// Pop the next item from the queue
    pub fn pop(&mut self) -> Option<ConsumableItem<T>> {
        self.rx.pop().map(ConsumableItem)
    }

    /// Push an item back to the producer for recycling
    pub fn push_back(&mut self, consumed_item: ConsumableItem<T>) {
        let ConsumableItem(node) = consumed_item;
        self.tx.push(node);
    }

    /// Consume all pending items by pushing them back to the producer
    pub fn drain(&mut self) {
        while let Some(node) = self.rx.pop() {
            self.tx.push(node);
        }
    }
}
