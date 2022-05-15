use jni::objects::JObject;

pub struct Friend<'a> {
    pub(crate) inner: JObject<'a>,
}

impl<'a> Friend<'a> {
    pub fn a(&self) {
        let _ = self.inner;
    }
}