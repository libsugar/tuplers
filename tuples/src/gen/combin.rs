// auto generated code, do not modify

impl<T, T0, T1> CombinLeft<T> for (T0, T1) {
    type Out = (T, T0, T1);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1)
    }
}
impl<T, T0, T1> CombinRight<T> for (T0, T1) {
    type Out = (T0, T1, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, target)
    }
}
impl<T, T0, T1, T2> CombinLeft<T> for (T0, T1, T2) {
    type Out = (T, T0, T1, T2);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2)
    }
}
impl<T, T0, T1, T2> CombinRight<T> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, target)
    }
}
impl<T, T0, T1, T2, T3> CombinLeft<T> for (T0, T1, T2, T3) {
    type Out = (T, T0, T1, T2, T3);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3)
    }
}
impl<T, T0, T1, T2, T3> CombinRight<T> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, target)
    }
}
impl<T, T0, T1, T2, T3, T4> CombinLeft<T> for (T0, T1, T2, T3, T4) {
    type Out = (T, T0, T1, T2, T3, T4);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4)
    }
}
impl<T, T0, T1, T2, T3, T4> CombinRight<T> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5> CombinLeft<T> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T, T0, T1, T2, T3, T4, T5);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5)
    }
}
impl<T, T0, T1, T2, T3, T4, T5> CombinRight<T> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, target)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> CombinLeft<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Out = (T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn push_left(self, target: T) -> Self::Out {
        (target, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> CombinRight<T> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T);
    fn push_right(self, target: T) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31, target)
    }
}
impl CombinConcat<()> for () {
    type Out = ();
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        ()
    }
}
impl<T0> CombinConcat<(T0,)> for () {
    type Out = (T0,);
    #[allow(unused_variables)]
    fn concat(self, target: (T0,)) -> Self::Out {
        (target.0,)
    }
}
impl<T0, T1> CombinConcat<(T0, T1)> for () {
    type Out = (T0, T1);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1)) -> Self::Out {
        (target.0, target.1)
    }
}
impl<T0, T1, T2> CombinConcat<(T0, T1, T2)> for () {
    type Out = (T0, T1, T2);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2)) -> Self::Out {
        (target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3> CombinConcat<(T0, T1, T2, T3)> for () {
    type Out = (T0, T1, T2, T3);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3)) -> Self::Out {
        (target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<(T0, T1, T2, T3, T4)> for () {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T0, T1, T2, T3, T4, T5)> for () {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T0, T1, T2, T3, T4, T5, T6)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for () {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0> CombinConcat<()> for (T0,) {
    type Out = (T0,);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0,)
    }
}
impl<T0, T1> CombinConcat<(T1,)> for (T0,) {
    type Out = (T0, T1);
    #[allow(unused_variables)]
    fn concat(self, target: (T1,)) -> Self::Out {
        (self.0, target.0)
    }
}
impl<T0, T1, T2> CombinConcat<(T1, T2)> for (T0,) {
    type Out = (T0, T1, T2);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2)) -> Self::Out {
        (self.0, target.0, target.1)
    }
}
impl<T0, T1, T2, T3> CombinConcat<(T1, T2, T3)> for (T0,) {
    type Out = (T0, T1, T2, T3);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3)) -> Self::Out {
        (self.0, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<(T1, T2, T3, T4)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T1, T2, T3, T4, T5)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T1, T2, T3, T4, T5, T6)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T1, T2, T3, T4, T5, T6, T7)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0,) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1> CombinConcat<()> for (T0, T1) {
    type Out = (T0, T1);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1)
    }
}
impl<T0, T1, T2> CombinConcat<(T2,)> for (T0, T1) {
    type Out = (T0, T1, T2);
    #[allow(unused_variables)]
    fn concat(self, target: (T2,)) -> Self::Out {
        (self.0, self.1, target.0)
    }
}
impl<T0, T1, T2, T3> CombinConcat<(T2, T3)> for (T0, T1) {
    type Out = (T0, T1, T2, T3);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3)) -> Self::Out {
        (self.0, self.1, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<(T2, T3, T4)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T2, T3, T4, T5)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T2, T3, T4, T5, T6)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T2, T3, T4, T5, T6, T7)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T2, T3, T4, T5, T6, T7, T8)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2> CombinConcat<()> for (T0, T1, T2) {
    type Out = (T0, T1, T2);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2)
    }
}
impl<T0, T1, T2, T3> CombinConcat<(T3,)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3);
    #[allow(unused_variables)]
    fn concat(self, target: (T3,)) -> Self::Out {
        (self.0, self.1, self.2, target.0)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<(T3, T4)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T3, T4, T5)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T3, T4, T5, T6)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T3, T4, T5, T6, T7)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T3, T4, T5, T6, T7, T8)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T3, T4, T5, T6, T7, T8, T9)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3> CombinConcat<()> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<(T4,)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: (T4,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T4, T5)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T4, T5, T6)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T4, T5, T6, T7)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T4, T5, T6, T7, T8)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T4, T5, T6, T7, T8, T9)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T4, T5, T6, T7, T8, T9, T10)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4> CombinConcat<()> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<(T5,)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: (T5,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T5, T6)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T5, T6, T7)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T5, T6, T7, T8)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T5, T6, T7, T8, T9)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T5, T6, T7, T8, T9, T10)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T5, T6, T7, T8, T9, T10, T11)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5> CombinConcat<()> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<(T6,)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: (T6,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T6, T7)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T6, T7, T8)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T6, T7, T8, T9)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T6, T7, T8, T9, T10)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T6, T7, T8, T9, T10, T11)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T6, T7, T8, T9, T10, T11, T12)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<(T7,)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: (T7,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T7, T8)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T7, T8, T9)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T7, T8, T9, T10)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T7, T8, T9, T10, T11)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T7, T8, T9, T10, T11, T12)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T7, T8, T9, T10, T11, T12, T13)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<(T8,)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: (T8,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T8, T9)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T8, T9, T10)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T8, T9, T10, T11)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T8, T9, T10, T11, T12)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T8, T9, T10, T11, T12, T13)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T8, T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<(T9,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: (T9,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T9, T10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T9, T10, T11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T9, T10, T11, T12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T9, T10, T11, T12, T13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T9, T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T9, T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<(T10,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: (T10,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T10, T11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T10, T11, T12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T10, T11, T12, T13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T10, T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T10, T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T10, T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<(T11,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: (T11,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T11, T12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T11, T12, T13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T11, T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T11, T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T11, T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T11, T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<(T12,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: (T12,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T12, T13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T12, T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T12, T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T12, T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T12, T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T12, T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinConcat<(T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    #[allow(unused_variables)]
    fn concat(self, target: (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<(T13,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: (T13,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T13, T14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T13, T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T13, T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T13, T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T13, T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T13, T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinConcat<(T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    #[allow(unused_variables)]
    fn concat(self, target: (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<(T14,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: (T14,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T14, T15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T14, T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T14, T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T14, T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T14, T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T14, T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> CombinConcat<(T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    #[allow(unused_variables)]
    fn concat(self, target: (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<(T15,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: (T15,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T15, T16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T15, T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T15, T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T15, T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T15, T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T15, T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> CombinConcat<(T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    #[allow(unused_variables)]
    fn concat(self, target: (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> CombinConcat<()> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    #[allow(unused_variables)]
    fn concat(self, target: ()) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> CombinConcat<(T16,)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    #[allow(unused_variables)]
    fn concat(self, target: (T16,)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> CombinConcat<(T16, T17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> CombinConcat<(T16, T17, T18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> CombinConcat<(T16, T17, T18, T19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> CombinConcat<(T16, T17, T18, T19, T20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> CombinConcat<(T16, T17, T18, T19, T20, T21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> CombinConcat<(T16, T17, T18, T19, T20, T21, T22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> CombinConcat<(T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Out = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    #[allow(unused_variables)]
    fn concat(self, target: (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Out {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, target.0, target.1, target.2, target.3, target.4, target.5, target.6, target.7, target.8, target.9, target.10, target.11, target.12, target.13, target.14, target.15)
    }
}
