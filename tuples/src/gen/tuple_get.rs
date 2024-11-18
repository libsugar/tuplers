// auto generated code, do not modify

impl<T> TupleGet for (T,) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 1usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T,) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 1usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T,) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 1usize || b >= 1usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 1usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 1usize || b >= 1usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 2usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 2usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 2usize || b >= 2usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 2usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 2usize || b >= 2usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 3usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 3usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 3usize || b >= 3usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 3usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 3usize || b >= 3usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 4usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 4usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 4usize || b >= 4usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 4usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 4usize || b >= 4usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 5usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 5usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 5usize || b >= 5usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 5usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 5usize || b >= 5usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 6usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 6usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 6usize || b >= 6usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 6usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 6usize || b >= 6usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 7usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 7usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 7usize || b >= 7usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 7usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 7usize || b >= 7usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 8usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 8usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 8usize || b >= 8usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 8usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 8usize || b >= 8usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 9usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 9usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 9usize || b >= 9usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 9usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 9usize || b >= 9usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 10usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 10usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 10usize || b >= 10usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 10usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 10usize || b >= 10usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 11usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 11usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 11usize || b >= 11usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 11usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 11usize || b >= 11usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 12usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 12usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 12usize || b >= 12usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 12usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 12usize || b >= 12usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 13usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 13usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 13usize || b >= 13usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 13usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 13usize || b >= 13usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 14usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 14usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 14usize || b >= 14usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 14usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 14usize || b >= 14usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 15usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 15usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 15usize || b >= 15usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 15usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 15usize || b >= 15usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 16usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 16usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 16usize || b >= 16usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 16usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 16usize || b >= 16usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 17usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 17usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 17usize || b >= 17usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 17usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 17usize || b >= 17usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 18usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 18usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 18usize || b >= 18usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 18usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 18usize || b >= 18usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 19usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 19usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 19usize || b >= 19usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 19usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 19usize || b >= 19usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 20usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 20usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 20usize || b >= 20usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 20usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 20usize || b >= 20usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 21usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 21usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 21usize || b >= 21usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 21usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 21usize || b >= 21usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 22usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 22usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 22usize || b >= 22usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 22usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 22usize || b >= 22usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 23usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 23usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 23usize || b >= 23usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 23usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 23usize || b >= 23usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 24usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 24usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 24usize || b >= 24usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 24usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 24usize || b >= 24usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 25usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 25usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 25usize || b >= 25usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 25usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 25usize || b >= 25usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 26usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 26usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 26usize || b >= 26usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 26usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 26usize || b >= 26usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 27usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 27usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 27usize || b >= 27usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 27usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 27usize || b >= 27usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            27 => &self.27,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 28usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            27 => Some(&self.27),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            27 => &mut self.27,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 28usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            27 => Some(&mut self.27),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 28usize || b >= 28usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 28usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 28usize || b >= 28usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            27 => &self.27,
            28 => &self.28,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 29usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            27 => Some(&self.27),
            28 => Some(&self.28),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            27 => &mut self.27,
            28 => &mut self.28,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 29usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            27 => Some(&mut self.27),
            28 => Some(&mut self.28),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 29usize || b >= 29usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 29usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 29usize || b >= 29usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            27 => &self.27,
            28 => &self.28,
            29 => &self.29,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 30usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            27 => Some(&self.27),
            28 => Some(&self.28),
            29 => Some(&self.29),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            27 => &mut self.27,
            28 => &mut self.28,
            29 => &mut self.29,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 30usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            27 => Some(&mut self.27),
            28 => Some(&mut self.28),
            29 => Some(&mut self.29),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 30usize || b >= 30usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 30usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 30usize || b >= 30usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            27 => &self.27,
            28 => &self.28,
            29 => &self.29,
            30 => &self.30,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 31usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            27 => Some(&self.27),
            28 => Some(&self.28),
            29 => Some(&self.29),
            30 => Some(&self.30),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            27 => &mut self.27,
            28 => &mut self.28,
            29 => &mut self.29,
            30 => &mut self.30,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 31usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            27 => Some(&mut self.27),
            28 => Some(&mut self.28),
            29 => Some(&mut self.29),
            30 => Some(&mut self.30),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 31usize || b >= 31usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 31usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 31usize || b >= 31usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
impl<T> TupleGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn get(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            4 => &self.4,
            5 => &self.5,
            6 => &self.6,
            7 => &self.7,
            8 => &self.8,
            9 => &self.9,
            10 => &self.10,
            11 => &self.11,
            12 => &self.12,
            13 => &self.13,
            14 => &self.14,
            15 => &self.15,
            16 => &self.16,
            17 => &self.17,
            18 => &self.18,
            19 => &self.19,
            20 => &self.20,
            21 => &self.21,
            22 => &self.22,
            23 => &self.23,
            24 => &self.24,
            25 => &self.25,
            26 => &self.26,
            27 => &self.27,
            28 => &self.28,
            29 => &self.29,
            30 => &self.30,
            31 => &self.31,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 32usize, index),
        }
    }
    fn try_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            5 => Some(&self.5),
            6 => Some(&self.6),
            7 => Some(&self.7),
            8 => Some(&self.8),
            9 => Some(&self.9),
            10 => Some(&self.10),
            11 => Some(&self.11),
            12 => Some(&self.12),
            13 => Some(&self.13),
            14 => Some(&self.14),
            15 => Some(&self.15),
            16 => Some(&self.16),
            17 => Some(&self.17),
            18 => Some(&self.18),
            19 => Some(&self.19),
            20 => Some(&self.20),
            21 => Some(&self.21),
            22 => Some(&self.22),
            23 => Some(&self.23),
            24 => Some(&self.24),
            25 => Some(&self.25),
            26 => Some(&self.26),
            27 => Some(&self.27),
            28 => Some(&self.28),
            29 => Some(&self.29),
            30 => Some(&self.30),
            31 => Some(&self.31),
            _ => None,
        }
    }
}
impl<T> TupleGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn get_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            4 => &mut self.4,
            5 => &mut self.5,
            6 => &mut self.6,
            7 => &mut self.7,
            8 => &mut self.8,
            9 => &mut self.9,
            10 => &mut self.10,
            11 => &mut self.11,
            12 => &mut self.12,
            13 => &mut self.13,
            14 => &mut self.14,
            15 => &mut self.15,
            16 => &mut self.16,
            17 => &mut self.17,
            18 => &mut self.18,
            19 => &mut self.19,
            20 => &mut self.20,
            21 => &mut self.21,
            22 => &mut self.22,
            23 => &mut self.23,
            24 => &mut self.24,
            25 => &mut self.25,
            26 => &mut self.26,
            27 => &mut self.27,
            28 => &mut self.28,
            29 => &mut self.29,
            30 => &mut self.30,
            31 => &mut self.31,
            _ => panic!("index out of bounds: the len is {} but the index is {}", 32usize, index),
        }
    }
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            5 => Some(&mut self.5),
            6 => Some(&mut self.6),
            7 => Some(&mut self.7),
            8 => Some(&mut self.8),
            9 => Some(&mut self.9),
            10 => Some(&mut self.10),
            11 => Some(&mut self.11),
            12 => Some(&mut self.12),
            13 => Some(&mut self.13),
            14 => Some(&mut self.14),
            15 => Some(&mut self.15),
            16 => Some(&mut self.16),
            17 => Some(&mut self.17),
            18 => Some(&mut self.18),
            19 => Some(&mut self.19),
            20 => Some(&mut self.20),
            21 => Some(&mut self.21),
            22 => Some(&mut self.22),
            23 => Some(&mut self.23),
            24 => Some(&mut self.24),
            25 => Some(&mut self.25),
            26 => Some(&mut self.26),
            27 => Some(&mut self.27),
            28 => Some(&mut self.28),
            29 => Some(&mut self.29),
            30 => Some(&mut self.30),
            31 => Some(&mut self.31),
            _ => None,
        }
    }
}
impl<T> TupleSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn swap(&mut self, a: usize, b: usize) {
        if a >= 32usize || b >= 32usize {
            panic!("index out of bounds: the len is {} but the index is {} and {}", 32usize, a, b);
        }
        if a == b {
            return;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
    }
    fn try_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 32usize || b >= 32usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let this = self as *mut Self;
            let a = (&mut *this).get_mut(a);
            let b = (&mut *this).get_mut(b);
            core::mem::swap(a, b);
        }
        true
    }
}
