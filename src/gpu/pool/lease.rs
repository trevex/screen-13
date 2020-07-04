use {
    super::PoolRef,
    std::ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Lease<T> {
    item: Option<T>,
    pool: PoolRef<T>,
}

impl<T> Lease<T> {
    pub(super) fn new(item: T, pool: &PoolRef<T>) -> Self {
        Self {
            item: Some(item),
            pool: PoolRef::clone(pool),
        }
    }
}

impl<T> AsMut<T> for Lease<T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> AsRef<T> for Lease<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Deref for Lease<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.item.as_ref().unwrap()
    }
}

impl<T> DerefMut for Lease<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
}

impl<T> Drop for Lease<T> {
    fn drop(&mut self) {
        self.pool
            .as_ref()
            .borrow_mut()
            .push_front(self.item.take().unwrap());
    }
}
