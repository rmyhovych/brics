pub struct RcMut<T> {
    obj: std::rc::Rc<Box<T>>,
}

impl<T> RcMut<T> {
    pub fn new(obj: T) -> Self {
        Self {
            obj: std::rc::Rc::new(Box::new(obj)),
        }
    }
}

impl<T> std::ops::Deref for RcMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.obj.as_ref().as_ref()
    }
}

impl<T> std::ops::DerefMut for RcMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let reference: &T = self.obj.as_ref().as_ref();
        unsafe { (reference as *const T as *mut T).as_mut().unwrap() }
    }
}
