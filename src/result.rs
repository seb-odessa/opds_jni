use jni::{
    objects::{JObject, JValueGen},
    sys::jobject,
    JNIEnv,
};
use opds_api::{Author, Value};

pub struct JavaObject {
    pub ptr: jobject,
}
impl JavaObject {
    fn success(env: &mut JNIEnv, object: JObject) -> anyhow::Result<Self> {
        let args = [JValueGen::from(&object)];
        let result = env.find_class("Wrapper$Result")?;
        let ptr = *env
            .call_static_method(
                result,
                "success",
                "(Ljava/lang/Object;)LWrapper$Result;",
                &args,
            )?
            .l()?;

        Ok(Self { ptr })
    }
    fn error(env: &mut JNIEnv, object: JObject) -> anyhow::Result<Self> {
        let args = [JValueGen::from(&object)];
        let result = env.find_class("Wrapper$Result")?;
        let ptr = *env
            .call_static_method(
                result,
                "error",
                "(Ljava/lang/String;)LWrapper$Result;",
                &args,
            )?
            .l()?;

        Ok(Self { ptr })
    }

    fn value<'a>(env: &mut JNIEnv<'a>, item: &Value) -> anyhow::Result<JObject<'a>> {
        let id = item.id as i32;
        let value = JObject::from(env.new_string(item.value.clone())?);
        let args = [JValueGen::from(id), JValueGen::from(&value)];
        let class = env.find_class("Value")?;
        let obj = env.new_object(class, "(ILjava/lang/String;)V", &args)?;
        Ok(obj)
    }

    fn author<'a>(env: &mut JNIEnv<'a>, item: &Author) -> anyhow::Result<JObject<'a>> {
        let fname = Self::value(env, &item.first_name)?;
        let mname = Self::value(env, &item.middle_name)?;
        let lname = Self::value(env, &item.last_name)?;

        let args = [
            JValueGen::from(&fname),
            JValueGen::from(&mname),
            JValueGen::from(&lname),
        ];
        let class = env.find_class("Author")?;
        let obj = env.new_object(class, "(LValue;LValue;LValue;)V", &args)?;
        Ok(obj)
    }
}
impl TryFrom<(&mut JNIEnv<'_>, anyhow::Error)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, err): (&mut JNIEnv<'_>, anyhow::Error)) -> anyhow::Result<Self> {
        let msg = env.new_string(format!("Err: {err}"))?;
        let object = JObject::from(msg);
        Self::error(env, object)
    }
}
impl TryFrom<(JNIEnv<'_>, Vec<String>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((mut env, items): (JNIEnv<'_>, Vec<String>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items {
            let object = JObject::from(env.new_string(item)?);
            let args = [JValueGen::from(&object)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(&mut env, list)
    }
}
impl TryFrom<(JNIEnv<'_>, Vec<Value>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((mut env, items): (JNIEnv<'_>, Vec<Value>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let value = Self::value(&mut env, item)?;
            let args = [JValueGen::from(&value)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(&mut env, list)
    }
}
impl TryFrom<(JNIEnv<'_>, Vec<Author>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((mut env, items): (JNIEnv<'_>, Vec<Author>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let obj = Self::author(&mut env, item)?;
            let args = [JValueGen::from(&obj)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(&mut env, list)
    }
}
