pub mod plain_text {
    use std::mem;

    use jni::JNIEnv;
    use jni::objects::{GlobalRef, JValue};

    use crate::jni_ffi::jni_callback::MIRAI_ENV;
    use crate::RawString;

    pub fn new_unchecked(env: JNIEnv, content: RawString) -> jni::errors::Result<GlobalRef> {
        let mirai = MIRAI_ENV.get().unwrap();

        let content = String::from(content);

        let jstr = env.new_string(&content)?;

        //let plain_text_class = env.find_class("net/mamoe/mirai/message/data/PlainText")?;
        let o = env.new_object_unchecked(
            &mirai.plain_text_class,
            mirai.new_plain_text,
            &[JValue::Object(*jstr)],
        )?;

        mem::forget(content);

        env.new_global_ref(o)
    }
}