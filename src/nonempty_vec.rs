pub struct NonEmptyVec<T> {
    pub vec: Vec<T>,
    _private: (),
}

impl<T> NonEmptyVec<T> {
    pub fn try_new(vec: Vec<T>) -> Option<NonEmptyVec<T>> {
        if vec.is_empty() {
            None
        } else {
            Some(NonEmptyVec {
                vec,
                _private: (),
            })
        }
    }

    pub fn first(&self) -> &T {
        self.vec.first().unwrap()
    }

    pub fn last(&self) -> &T {
        self.vec.last().unwrap()
    }

    
}