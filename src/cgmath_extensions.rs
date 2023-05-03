use cgmath::{vec2, BaseNum, Vector2};

pub trait Decomposable<S: BaseNum> {
    fn x_component(self) -> Vector2<S>;
    fn y_component(self) -> Vector2<S>;
}

impl<S: BaseNum> Decomposable<S> for Vector2<S> {
    #[inline]
    fn x_component(self) -> Vector2<S> {
        return vec2(self.x, S::zero());
    }

    #[inline]
    fn y_component(self) -> Vector2<S> {
        return vec2(S::zero(), self.y);
    }
}
