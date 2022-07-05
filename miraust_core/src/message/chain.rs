pub mod builder {
    use jni::JNIEnv;
    use jni::objects::{GlobalRef, JValue};
    use jni::signature::JavaType;
    use jni::sys::jint;

    use crate::jni_ffi::jni_callback::MIRAI_ENV;

    pub unsafe fn new_unchecked(env: JNIEnv, initial_size: jint) -> jni::errors::Result<GlobalRef> {
        let mirai = MIRAI_ENV.get().unwrap();

        //let message_chain_builder_class = env.find_class("net/mamoe/mirai/message/data/MessageChainBuilder")?;
        let o = env.new_object_unchecked(
            &mirai.message_chain_builder_class,
            mirai.new_message_chain_builder,
            &[JValue::Int(initial_size)],
        );
        env.exception_describe().unwrap();

        env.new_global_ref(o?)
    }

    pub unsafe fn add_unchecked(env: JNIEnv, chain: GlobalRef, single: GlobalRef) -> bool {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(val) = env.call_method_unchecked(
            chain.as_obj(),
            mirai.message_chain_builder_add,
            JavaType::Object(String::new()),
            &[JValue::Object(single.as_obj())],
        ) {
            val.z().unwrap_or(false)
        } else { false }
    }

    pub unsafe fn as_message_chain_unchecked(env: JNIEnv, chain: GlobalRef) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(val) = env.call_method_unchecked(
            chain.as_obj(),
            mirai.message_chain_builder_as_message_chain,
            JavaType::Object(String::new()),
            &[],
        ) {
            let obj = val.l().ok()?;

            env.new_global_ref(obj).ok()
        } else { None }
    }
}

pub mod iterator {
    use jni::JNIEnv;
    use jni::objects::{GlobalRef, JValue};
    use jni::signature::JavaType;
    use jni::signature::Primitive::Boolean;
    use crate::jni_ffi::jni_callback::MIRAI_ENV;

    pub unsafe fn iterator_unchecked(env: JNIEnv, chain: GlobalRef) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(val) = env.call_static_method_unchecked(
            &mirai.chain_iterator_class,
            mirai.chain_iterator_get_iterator,
            JavaType::Object(String::new()),
            &[JValue::Object(chain.as_obj())],
        ) {
            let obj = val.l().ok()?;

            env.new_global_ref(obj).ok()
        } else { None }
    }

    pub unsafe fn iterator_has_next(env: JNIEnv, iter: GlobalRef) -> bool {
        let mirai = MIRAI_ENV.get().unwrap();

        if let Ok(val) = env.call_method_unchecked(
            iter.as_obj(),
            mirai.chain_iterator_has_next,
            JavaType::Primitive(Boolean),
            &[]
        ) {
            val.z().unwrap_or(false)
        } else { false }
    }
    
    pub unsafe fn iterator_next(env: JNIEnv, iter: GlobalRef) -> Option<GlobalRef> {
        let mirai = MIRAI_ENV.get().unwrap();
        
        if let Ok(val) = env.call_method_unchecked(
            iter.as_obj(),
            mirai.chain_iterator_next,
            JavaType::Object(String::new()),
            &[]
        ) {
            let obj = val.l().ok()?;
            
            env.new_global_ref(obj).ok()
        } else { None }
    }
}