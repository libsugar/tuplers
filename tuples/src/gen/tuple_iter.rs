// auto generated code, do not modify

#[derive(Debug, Clone)]
pub struct Tuple2Iter<'a, T>([&'a T; 2], Range<usize>);
impl<'a, T> Tuple2Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T)) -> Self {
        Self([&t.0, &t.1], 0..2)
    }
}
impl<'a, T> Iterator for Tuple2Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple2Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple2Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple2Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T) {
    type Iter = Tuple2Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple2Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple2IntoIter<T>([MaybeUninit<T>; 2], Range<usize>);
impl<T> Tuple2IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1)], 0..2)
    }
}
impl<T> Iterator for Tuple2IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple2IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple2IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple2IntoIter<T> {}
impl<T> TupleIntoIter for (T, T) {
    type Iter = Tuple2IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple2IntoIter::new(self)
    }
}
impl<T> Drop for Tuple2IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T) {
    type OutTuple = (Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple3Iter<'a, T>([&'a T; 3], Range<usize>);
impl<'a, T> Tuple3Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2], 0..3)
    }
}
impl<'a, T> Iterator for Tuple3Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple3Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple3Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple3Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T) {
    type Iter = Tuple3Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple3Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple3IntoIter<T>([MaybeUninit<T>; 3], Range<usize>);
impl<T> Tuple3IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2)], 0..3)
    }
}
impl<T> Iterator for Tuple3IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple3IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple3IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple3IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T) {
    type Iter = Tuple3IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple3IntoIter::new(self)
    }
}
impl<T> Drop for Tuple3IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple4Iter<'a, T>([&'a T; 4], Range<usize>);
impl<'a, T> Tuple4Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3], 0..4)
    }
}
impl<'a, T> Iterator for Tuple4Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple4Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple4Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple4Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T) {
    type Iter = Tuple4Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple4Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple4IntoIter<T>([MaybeUninit<T>; 4], Range<usize>);
impl<T> Tuple4IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3)], 0..4)
    }
}
impl<T> Iterator for Tuple4IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple4IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple4IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple4IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T) {
    type Iter = Tuple4IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple4IntoIter::new(self)
    }
}
impl<T> Drop for Tuple4IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple5Iter<'a, T>([&'a T; 5], Range<usize>);
impl<'a, T> Tuple5Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4], 0..5)
    }
}
impl<'a, T> Iterator for Tuple5Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple5Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple5Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple5Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T) {
    type Iter = Tuple5Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple5Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple5IntoIter<T>([MaybeUninit<T>; 5], Range<usize>);
impl<T> Tuple5IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4)], 0..5)
    }
}
impl<T> Iterator for Tuple5IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple5IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple5IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple5IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T) {
    type Iter = Tuple5IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple5IntoIter::new(self)
    }
}
impl<T> Drop for Tuple5IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple6Iter<'a, T>([&'a T; 6], Range<usize>);
impl<'a, T> Tuple6Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5], 0..6)
    }
}
impl<'a, T> Iterator for Tuple6Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple6Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple6Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple6Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T) {
    type Iter = Tuple6Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple6Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple6IntoIter<T>([MaybeUninit<T>; 6], Range<usize>);
impl<T> Tuple6IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5)], 0..6)
    }
}
impl<T> Iterator for Tuple6IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple6IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple6IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple6IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T) {
    type Iter = Tuple6IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple6IntoIter::new(self)
    }
}
impl<T> Drop for Tuple6IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple7Iter<'a, T>([&'a T; 7], Range<usize>);
impl<'a, T> Tuple7Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6], 0..7)
    }
}
impl<'a, T> Iterator for Tuple7Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple7Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple7Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple7Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T) {
    type Iter = Tuple7Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple7Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple7IntoIter<T>([MaybeUninit<T>; 7], Range<usize>);
impl<T> Tuple7IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6)], 0..7)
    }
}
impl<T> Iterator for Tuple7IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple7IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple7IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple7IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T) {
    type Iter = Tuple7IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple7IntoIter::new(self)
    }
}
impl<T> Drop for Tuple7IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple8Iter<'a, T>([&'a T; 8], Range<usize>);
impl<'a, T> Tuple8Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7], 0..8)
    }
}
impl<'a, T> Iterator for Tuple8Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple8Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple8Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple8Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T) {
    type Iter = Tuple8Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple8Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple8IntoIter<T>([MaybeUninit<T>; 8], Range<usize>);
impl<T> Tuple8IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7)], 0..8)
    }
}
impl<T> Iterator for Tuple8IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple8IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple8IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple8IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T) {
    type Iter = Tuple8IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple8IntoIter::new(self)
    }
}
impl<T> Drop for Tuple8IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple9Iter<'a, T>([&'a T; 9], Range<usize>);
impl<'a, T> Tuple9Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8], 0..9)
    }
}
impl<'a, T> Iterator for Tuple9Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple9Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple9Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple9Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple9Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple9Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple9IntoIter<T>([MaybeUninit<T>; 9], Range<usize>);
impl<T> Tuple9IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8)], 0..9)
    }
}
impl<T> Iterator for Tuple9IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple9IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple9IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple9IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple9IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple9IntoIter::new(self)
    }
}
impl<T> Drop for Tuple9IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple10Iter<'a, T>([&'a T; 10], Range<usize>);
impl<'a, T> Tuple10Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9], 0..10)
    }
}
impl<'a, T> Iterator for Tuple10Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple10Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple10Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple10Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple10Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple10Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple10IntoIter<T>([MaybeUninit<T>; 10], Range<usize>);
impl<T> Tuple10IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9)], 0..10)
    }
}
impl<T> Iterator for Tuple10IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple10IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple10IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple10IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple10IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple10IntoIter::new(self)
    }
}
impl<T> Drop for Tuple10IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple11Iter<'a, T>([&'a T; 11], Range<usize>);
impl<'a, T> Tuple11Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10], 0..11)
    }
}
impl<'a, T> Iterator for Tuple11Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple11Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple11Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple11Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple11Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple11Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple11IntoIter<T>([MaybeUninit<T>; 11], Range<usize>);
impl<T> Tuple11IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10)], 0..11)
    }
}
impl<T> Iterator for Tuple11IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple11IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple11IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple11IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple11IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple11IntoIter::new(self)
    }
}
impl<T> Drop for Tuple11IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
#[derive(Debug, Clone)]
pub struct Tuple12Iter<'a, T>([&'a T; 12], Range<usize>);
impl<'a, T> Tuple12Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11], 0..12)
    }
}
impl<'a, T> Iterator for Tuple12Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple12Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple12Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple12Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple12Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple12Iter::new(self)
    }
}
#[derive(Debug)]
pub struct Tuple12IntoIter<T>([MaybeUninit<T>; 12], Range<usize>);
impl<T> Tuple12IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11)], 0..12)
    }
}
impl<T> Iterator for Tuple12IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple12IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple12IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple12IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple12IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple12IntoIter::new(self)
    }
}
impl<T> Drop for Tuple12IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple13Iter<'a, T>([&'a T; 13], Range<usize>);
impl<'a, T> Tuple13Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12], 0..13)
    }
}
impl<'a, T> Iterator for Tuple13Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple13Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple13Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple13Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple13Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple13Iter::new(self)
    }
}
pub struct Tuple13IntoIter<T>([MaybeUninit<T>; 13], Range<usize>);
impl<T> Tuple13IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12)], 0..13)
    }
}
impl<T> Iterator for Tuple13IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple13IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple13IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple13IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple13IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple13IntoIter::new(self)
    }
}
impl<T> Drop for Tuple13IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple14Iter<'a, T>([&'a T; 14], Range<usize>);
impl<'a, T> Tuple14Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13], 0..14)
    }
}
impl<'a, T> Iterator for Tuple14Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple14Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple14Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple14Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple14Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple14Iter::new(self)
    }
}
pub struct Tuple14IntoIter<T>([MaybeUninit<T>; 14], Range<usize>);
impl<T> Tuple14IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13)], 0..14)
    }
}
impl<T> Iterator for Tuple14IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple14IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple14IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple14IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple14IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple14IntoIter::new(self)
    }
}
impl<T> Drop for Tuple14IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple15Iter<'a, T>([&'a T; 15], Range<usize>);
impl<'a, T> Tuple15Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14], 0..15)
    }
}
impl<'a, T> Iterator for Tuple15Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple15Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple15Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple15Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple15Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple15Iter::new(self)
    }
}
pub struct Tuple15IntoIter<T>([MaybeUninit<T>; 15], Range<usize>);
impl<T> Tuple15IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14)], 0..15)
    }
}
impl<T> Iterator for Tuple15IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple15IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple15IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple15IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple15IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple15IntoIter::new(self)
    }
}
impl<T> Drop for Tuple15IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple16Iter<'a, T>([&'a T; 16], Range<usize>);
impl<'a, T> Tuple16Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15], 0..16)
    }
}
impl<'a, T> Iterator for Tuple16Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple16Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple16Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple16Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple16Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple16Iter::new(self)
    }
}
pub struct Tuple16IntoIter<T>([MaybeUninit<T>; 16], Range<usize>);
impl<T> Tuple16IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15)], 0..16)
    }
}
impl<T> Iterator for Tuple16IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple16IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple16IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple16IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple16IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple16IntoIter::new(self)
    }
}
impl<T> Drop for Tuple16IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple17Iter<'a, T>([&'a T; 17], Range<usize>);
impl<'a, T> Tuple17Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16], 0..17)
    }
}
impl<'a, T> Iterator for Tuple17Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple17Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple17Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple17Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple17Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple17Iter::new(self)
    }
}
pub struct Tuple17IntoIter<T>([MaybeUninit<T>; 17], Range<usize>);
impl<T> Tuple17IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16)], 0..17)
    }
}
impl<T> Iterator for Tuple17IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple17IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple17IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple17IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple17IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple17IntoIter::new(self)
    }
}
impl<T> Drop for Tuple17IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple18Iter<'a, T>([&'a T; 18], Range<usize>);
impl<'a, T> Tuple18Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17], 0..18)
    }
}
impl<'a, T> Iterator for Tuple18Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple18Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple18Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple18Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple18Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple18Iter::new(self)
    }
}
pub struct Tuple18IntoIter<T>([MaybeUninit<T>; 18], Range<usize>);
impl<T> Tuple18IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17)], 0..18)
    }
}
impl<T> Iterator for Tuple18IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple18IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple18IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple18IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple18IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple18IntoIter::new(self)
    }
}
impl<T> Drop for Tuple18IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple19Iter<'a, T>([&'a T; 19], Range<usize>);
impl<'a, T> Tuple19Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18], 0..19)
    }
}
impl<'a, T> Iterator for Tuple19Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple19Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple19Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple19Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple19Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple19Iter::new(self)
    }
}
pub struct Tuple19IntoIter<T>([MaybeUninit<T>; 19], Range<usize>);
impl<T> Tuple19IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18)], 0..19)
    }
}
impl<T> Iterator for Tuple19IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple19IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple19IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple19IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple19IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple19IntoIter::new(self)
    }
}
impl<T> Drop for Tuple19IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple20Iter<'a, T>([&'a T; 20], Range<usize>);
impl<'a, T> Tuple20Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19], 0..20)
    }
}
impl<'a, T> Iterator for Tuple20Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple20Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple20Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple20Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple20Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple20Iter::new(self)
    }
}
pub struct Tuple20IntoIter<T>([MaybeUninit<T>; 20], Range<usize>);
impl<T> Tuple20IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19)], 0..20)
    }
}
impl<T> Iterator for Tuple20IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple20IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple20IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple20IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple20IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple20IntoIter::new(self)
    }
}
impl<T> Drop for Tuple20IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple21Iter<'a, T>([&'a T; 21], Range<usize>);
impl<'a, T> Tuple21Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20], 0..21)
    }
}
impl<'a, T> Iterator for Tuple21Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple21Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple21Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple21Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple21Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple21Iter::new(self)
    }
}
pub struct Tuple21IntoIter<T>([MaybeUninit<T>; 21], Range<usize>);
impl<T> Tuple21IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20)], 0..21)
    }
}
impl<T> Iterator for Tuple21IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple21IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple21IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple21IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple21IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple21IntoIter::new(self)
    }
}
impl<T> Drop for Tuple21IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple22Iter<'a, T>([&'a T; 22], Range<usize>);
impl<'a, T> Tuple22Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21], 0..22)
    }
}
impl<'a, T> Iterator for Tuple22Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple22Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple22Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple22Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple22Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple22Iter::new(self)
    }
}
pub struct Tuple22IntoIter<T>([MaybeUninit<T>; 22], Range<usize>);
impl<T> Tuple22IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21)], 0..22)
    }
}
impl<T> Iterator for Tuple22IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple22IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple22IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple22IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple22IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple22IntoIter::new(self)
    }
}
impl<T> Drop for Tuple22IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple23Iter<'a, T>([&'a T; 23], Range<usize>);
impl<'a, T> Tuple23Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22], 0..23)
    }
}
impl<'a, T> Iterator for Tuple23Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple23Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple23Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple23Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple23Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple23Iter::new(self)
    }
}
pub struct Tuple23IntoIter<T>([MaybeUninit<T>; 23], Range<usize>);
impl<T> Tuple23IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22)], 0..23)
    }
}
impl<T> Iterator for Tuple23IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple23IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple23IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple23IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple23IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple23IntoIter::new(self)
    }
}
impl<T> Drop for Tuple23IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple24Iter<'a, T>([&'a T; 24], Range<usize>);
impl<'a, T> Tuple24Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23], 0..24)
    }
}
impl<'a, T> Iterator for Tuple24Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple24Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple24Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple24Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple24Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple24Iter::new(self)
    }
}
pub struct Tuple24IntoIter<T>([MaybeUninit<T>; 24], Range<usize>);
impl<T> Tuple24IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23)], 0..24)
    }
}
impl<T> Iterator for Tuple24IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple24IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple24IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple24IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple24IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple24IntoIter::new(self)
    }
}
impl<T> Drop for Tuple24IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple25Iter<'a, T>([&'a T; 25], Range<usize>);
impl<'a, T> Tuple25Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24], 0..25)
    }
}
impl<'a, T> Iterator for Tuple25Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple25Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple25Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple25Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple25Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple25Iter::new(self)
    }
}
pub struct Tuple25IntoIter<T>([MaybeUninit<T>; 25], Range<usize>);
impl<T> Tuple25IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24)], 0..25)
    }
}
impl<T> Iterator for Tuple25IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple25IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple25IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple25IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple25IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple25IntoIter::new(self)
    }
}
impl<T> Drop for Tuple25IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple26Iter<'a, T>([&'a T; 26], Range<usize>);
impl<'a, T> Tuple26Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25], 0..26)
    }
}
impl<'a, T> Iterator for Tuple26Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple26Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple26Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple26Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple26Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple26Iter::new(self)
    }
}
pub struct Tuple26IntoIter<T>([MaybeUninit<T>; 26], Range<usize>);
impl<T> Tuple26IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25)], 0..26)
    }
}
impl<T> Iterator for Tuple26IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple26IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple26IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple26IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple26IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple26IntoIter::new(self)
    }
}
impl<T> Drop for Tuple26IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple27Iter<'a, T>([&'a T; 27], Range<usize>);
impl<'a, T> Tuple27Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26], 0..27)
    }
}
impl<'a, T> Iterator for Tuple27Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple27Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple27Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple27Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple27Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple27Iter::new(self)
    }
}
pub struct Tuple27IntoIter<T>([MaybeUninit<T>; 27], Range<usize>);
impl<T> Tuple27IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26)], 0..27)
    }
}
impl<T> Iterator for Tuple27IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple27IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple27IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple27IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple27IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple27IntoIter::new(self)
    }
}
impl<T> Drop for Tuple27IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple28Iter<'a, T>([&'a T; 28], Range<usize>);
impl<'a, T> Tuple28Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26, &t.27], 0..28)
    }
}
impl<'a, T> Iterator for Tuple28Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple28Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple28Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple28Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple28Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple28Iter::new(self)
    }
}
pub struct Tuple28IntoIter<T>([MaybeUninit<T>; 28], Range<usize>);
impl<T> Tuple28IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26), MaybeUninit::new(t.27)], 0..28)
    }
}
impl<T> Iterator for Tuple28IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple28IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple28IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple28IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple28IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple28IntoIter::new(self)
    }
}
impl<T> Drop for Tuple28IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple29Iter<'a, T>([&'a T; 29], Range<usize>);
impl<'a, T> Tuple29Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26, &t.27, &t.28], 0..29)
    }
}
impl<'a, T> Iterator for Tuple29Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple29Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple29Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple29Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple29Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple29Iter::new(self)
    }
}
pub struct Tuple29IntoIter<T>([MaybeUninit<T>; 29], Range<usize>);
impl<T> Tuple29IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26), MaybeUninit::new(t.27), MaybeUninit::new(t.28)], 0..29)
    }
}
impl<T> Iterator for Tuple29IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple29IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple29IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple29IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple29IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple29IntoIter::new(self)
    }
}
impl<T> Drop for Tuple29IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple30Iter<'a, T>([&'a T; 30], Range<usize>);
impl<'a, T> Tuple30Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26, &t.27, &t.28, &t.29], 0..30)
    }
}
impl<'a, T> Iterator for Tuple30Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple30Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple30Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple30Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple30Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple30Iter::new(self)
    }
}
pub struct Tuple30IntoIter<T>([MaybeUninit<T>; 30], Range<usize>);
impl<T> Tuple30IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26), MaybeUninit::new(t.27), MaybeUninit::new(t.28), MaybeUninit::new(t.29)], 0..30)
    }
}
impl<T> Iterator for Tuple30IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple30IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple30IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple30IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple30IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple30IntoIter::new(self)
    }
}
impl<T> Drop for Tuple30IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple31Iter<'a, T>([&'a T; 31], Range<usize>);
impl<'a, T> Tuple31Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26, &t.27, &t.28, &t.29, &t.30], 0..31)
    }
}
impl<'a, T> Iterator for Tuple31Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple31Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple31Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple31Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple31Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple31Iter::new(self)
    }
}
pub struct Tuple31IntoIter<T>([MaybeUninit<T>; 31], Range<usize>);
impl<T> Tuple31IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26), MaybeUninit::new(t.27), MaybeUninit::new(t.28), MaybeUninit::new(t.29), MaybeUninit::new(t.30)], 0..31)
    }
}
impl<T> Iterator for Tuple31IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple31IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple31IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple31IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple31IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple31IntoIter::new(self)
    }
}
impl<T> Drop for Tuple31IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
pub struct Tuple32Iter<'a, T>([&'a T; 32], Range<usize>);
impl<'a, T> Tuple32Iter<'a, T> {
    #[inline]
    pub fn new(t: &'a (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([&t.0, &t.1, &t.2, &t.3, &t.4, &t.5, &t.6, &t.7, &t.8, &t.9, &t.10, &t.11, &t.12, &t.13, &t.14, &t.15, &t.16, &t.17, &t.18, &t.19, &t.20, &t.21, &t.22, &t.23, &t.24, &t.25, &t.26, &t.27, &t.28, &t.29, &t.30, &t.31], 0..32)
    }
}
impl<'a, T> Iterator for Tuple32Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<'a, T> DoubleEndedIterator for Tuple32Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
    }
}
impl<'a, T> ExactSizeIterator for Tuple32Iter<'a, T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<'a, T> FusedIterator for Tuple32Iter<'a, T> {}
impl<'a, T: 'a> TupleIter<'a> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple32Iter<'a, T>;
    #[inline]
    fn iter(&'a self) -> Self::Iter {
        Tuple32Iter::new(self)
    }
}
pub struct Tuple32IntoIter<T>([MaybeUninit<T>; 32], Range<usize>);
impl<T> Tuple32IntoIter<T> {
    #[inline]
    pub fn new(t: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self {
        Self([MaybeUninit::new(t.0), MaybeUninit::new(t.1), MaybeUninit::new(t.2), MaybeUninit::new(t.3), MaybeUninit::new(t.4), MaybeUninit::new(t.5), MaybeUninit::new(t.6), MaybeUninit::new(t.7), MaybeUninit::new(t.8), MaybeUninit::new(t.9), MaybeUninit::new(t.10), MaybeUninit::new(t.11), MaybeUninit::new(t.12), MaybeUninit::new(t.13), MaybeUninit::new(t.14), MaybeUninit::new(t.15), MaybeUninit::new(t.16), MaybeUninit::new(t.17), MaybeUninit::new(t.18), MaybeUninit::new(t.19), MaybeUninit::new(t.20), MaybeUninit::new(t.21), MaybeUninit::new(t.22), MaybeUninit::new(t.23), MaybeUninit::new(t.24), MaybeUninit::new(t.25), MaybeUninit::new(t.26), MaybeUninit::new(t.27), MaybeUninit::new(t.28), MaybeUninit::new(t.29), MaybeUninit::new(t.30), MaybeUninit::new(t.31)], 0..32)
    }
}
impl<T> Iterator for Tuple32IntoIter<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
    #[inline]
    fn count(self) -> usize {
        self.len()
    }
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}
impl<T> DoubleEndedIterator for Tuple32IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.1.next_back().map(|idx| unsafe { core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init() })
    }
}
impl<T> ExactSizeIterator for Tuple32IntoIter<T> {
    #[inline]
    fn len(&self) -> usize {
        self.1.end - self.1.start
    }
}
impl<T> FusedIterator for Tuple32IntoIter<T> {}
impl<T> TupleIntoIter for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Iter = Tuple32IntoIter<T>;
    #[inline]
    fn into_iter(self) -> Self::Iter {
        Tuple32IntoIter::new(self)
    }
}
impl<T> Drop for Tuple32IntoIter<T> {
    fn drop(&mut self) {
        let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
        let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
        unsafe { core::ptr::drop_in_place(slice) }
    }
}
impl<T> TupleFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> TupleTryFromIter<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?, iter.next()?))
    }
}
impl<T> TupleFromIterTry<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type OutTuple = (Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>, Option<T>);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next(), iter.next())
    }
}
