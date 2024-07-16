use jni::{
    objects::{JObject, JValueGen},
    sys::jobject,
    JNIEnv,
};
use opds_api::{Author, Book, Serie, Value};

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

    fn string<'a>(env: &mut JNIEnv<'a>, item: &String) -> anyhow::Result<JObject<'a>> {
        let obj = JObject::from(env.new_string(item.clone())?);
        Ok(obj)
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

    fn serie<'a>(env: &mut JNIEnv<'a>, item: &Serie) -> anyhow::Result<JObject<'a>> {
        let id = item.id as i32;
        let name = Self::string(env, &item.name)?;
        let count = item.count as i32;
        let author = Self::author(env, &item.author)?;

        let args = [
            JValueGen::from(id),
            JValueGen::from(&name),
            JValueGen::from(count),
            JValueGen::from(&author),
        ];
        let class = env.find_class("Serie")?;
        let obj = env.new_object(class, "(ILjava/lang/String;ILAuthor;)V", &args)?;
        Ok(obj)
    }

    fn book<'a>(env: &mut JNIEnv<'a>, item: &Book) -> anyhow::Result<JObject<'a>> {
        let id = item.id as i32;
        let name = Self::string(env, &item.name)?;
        let sid = if let Some(sid) = item.sid {
            sid as i32
        } else {
            0
        };
        let idx = if let Some(idx) = item.idx {
            idx as i32
        } else {
            0
        };
        let author = Self::author(env, &item.author)?;
        let size = item.size as i32;
        let added = Self::string(env, &item.added)?;

        let args = [
            JValueGen::from(id),
            JValueGen::from(&name),
            JValueGen::from(sid),
            JValueGen::from(idx),
            JValueGen::from(&author),
            JValueGen::from(size),
            JValueGen::from(&added),
        ];
        let class = env.find_class("Book")?;
        let obj = env.new_object(
            class,
            "(ILjava/lang/String;IILAuthor;ILjava/lang/String;)V",
            &args,
        )?;
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
impl TryFrom<(&mut JNIEnv<'_>, Vec<String>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, items): (&mut JNIEnv<'_>, Vec<String>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items {
            let object = JObject::from(env.new_string(item)?);
            let args = [JValueGen::from(&object)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(env, list)
    }
}
impl TryFrom<(&mut JNIEnv<'_>, Vec<Value>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, items): (&mut JNIEnv<'_>, Vec<Value>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let value = Self::value(env, item)?;
            let args = [JValueGen::from(&value)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(env, list)
    }
}
impl TryFrom<(&mut JNIEnv<'_>, Vec<Author>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, items): (&mut JNIEnv<'_>, Vec<Author>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let obj = Self::author(env, item)?;
            let args = [JValueGen::from(&obj)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(env, list)
    }
}
impl TryFrom<(&mut JNIEnv<'_>, Vec<Serie>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, items): (&mut JNIEnv<'_>, Vec<Serie>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let obj = Self::serie(env, item)?;
            let args = [JValueGen::from(&obj)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(env, list)
    }
}
impl TryFrom<(&mut JNIEnv<'_>, Vec<Book>)> for JavaObject {
    type Error = anyhow::Error;

    fn try_from((env, items): (&mut JNIEnv<'_>, Vec<Book>)) -> anyhow::Result<Self> {
        let class = env.find_class("java/util/ArrayList")?;
        let mut list = env.new_object(class, "()V", &[])?;

        for item in items.iter() {
            let obj = Self::book(env, item)?;
            let args = [JValueGen::from(&obj)];
            env.call_method(&mut list, "add", "(Ljava/lang/Object;)Z", &args)?;
        }

        Self::success(env, list)
    }
}
