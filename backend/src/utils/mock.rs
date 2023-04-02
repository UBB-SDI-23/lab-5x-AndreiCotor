use r2d2::Error;
use r2d2::{ManageConnection, PooledConnection};

pub enum Mockable<T> {
    Real(T),
    Mock
}

pub enum MockablePool<T: ManageConnection> {
    Real(r2d2::Pool<T>),
    Mock
}

impl<T: ManageConnection> MockablePool<T> {
    pub fn get(&self) -> Result<Mockable<PooledConnection<T>>, Error> {
        match self {
            MockablePool::Real(inner) => {
                match inner.get() {
                    Ok(val) => Ok(Mockable::Real(val)),
                    Err(error) => Err(error)
                }
            },
            MockablePool::Mock => Ok(Mockable::Mock)
        }
    }
}