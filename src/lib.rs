use std::marker::{PhantomData};
use std::ops::{Div};

#[derive(Clone, Copy, Debug)]
pub struct V {
    value: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct P<D> {
    value: f32,
    d: PhantomData<D>,
}

impl V {
    #[inline(always)]
    pub fn new(v: f32) -> V {
        V { value: v, }
    }

    #[inline(always)]
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

impl<D> P<D> {
    #[inline(always)]
    pub fn new(v: f32) -> P<D> {
        P { value: v, d: PhantomData, }
    }

    #[inline(always)]
    pub fn get_value(&self) -> f32 {
        self.value
    }
}

impl Div<V> for V {
    type Output = V;

    #[inline(always)]
    fn div(self, rhs: V) -> V {
        V { value: self.value / rhs.value, }
    }
}

impl<D> Div<P<D>> for P<D> {
    type Output = P<D>;

    #[inline(always)]
    fn div(self, rhs: P<D>) -> P<D> {
        P { value: self.value / rhs.value, d: PhantomData, }
    }
}
