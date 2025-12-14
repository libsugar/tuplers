// auto generated code, do not modify

impl<T> TupleDynamicGet for (T,) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            _ => None,
        }
    }
}
impl<T> TupleDynamicGetMut for (T,) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            _ => None,
        }
    }
}
impl<T> TupleDynamicSwap for (T,) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 1usize || b >= 1usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            _ => None,
        }
    }
}
impl<T> TupleDynamicGetMut for (T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            _ => None,
        }
    }
}
impl<T> TupleDynamicSwap for (T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 2usize || b >= 2usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            _ => None,
        }
    }
}
impl<T> TupleDynamicGetMut for (T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            _ => None,
        }
    }
}
impl<T> TupleDynamicSwap for (T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 3usize || b >= 3usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
        match index {
            0 => Some(&self.0),
            1 => Some(&self.1),
            2 => Some(&self.2),
            3 => Some(&self.3),
            _ => None,
        }
    }
}
impl<T> TupleDynamicGetMut for (T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
        match index {
            0 => Some(&mut self.0),
            1 => Some(&mut self.1),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            _ => None,
        }
    }
}
impl<T> TupleDynamicSwap for (T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 4usize || b >= 4usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 5usize || b >= 5usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 6usize || b >= 6usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 7usize || b >= 7usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 8usize || b >= 8usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 9usize || b >= 9usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 10usize || b >= 10usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 11usize || b >= 11usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 12usize || b >= 12usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 13usize || b >= 13usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 14usize || b >= 14usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 15usize || b >= 15usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 16usize || b >= 16usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 17usize || b >= 17usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 18usize || b >= 18usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 19usize || b >= 19usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 20usize || b >= 20usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 21usize || b >= 21usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 22usize || b >= 22usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 23usize || b >= 23usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 24usize || b >= 24usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 25usize || b >= 25usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 26usize || b >= 26usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 27usize || b >= 27usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 28usize || b >= 28usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 29usize || b >= 29usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 30usize || b >= 30usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 31usize || b >= 31usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
impl<T> TupleDynamicGet for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    type Output = T;
    fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
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
impl<T> TupleDynamicGetMut for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
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
impl<T> TupleDynamicSwap for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
        if a >= 32usize || b >= 32usize {
            return false;
        }
        if a == b {
            return true;
        }
        unsafe {
            let a = self.dyn_get_mut(a).unwrap() as *mut _;
            let b = self.dyn_get_mut(b).unwrap() as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
        true
    }
}
