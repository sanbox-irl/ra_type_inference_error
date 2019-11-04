#[derive(Debug, Clone, Default)]
pub struct Component<T> {
    pub is_active: bool,
    inner: T,
}
