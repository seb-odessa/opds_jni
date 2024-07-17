use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use opds_api::OpdsApi;
use result::JavaObject;

mod result;

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_createOpdsApi(mut env: JNIEnv, _: JClass, path: JString) -> jlong {
    let path: String = env
        .get_string(&path)
        .expect("Couldn't get java string!")
        .into();
    let api = OpdsApi::try_from(&path).expect("Failed to open database");
    Box::into_raw(Box::new(api)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_destroyOpdsApi(_: JNIEnv, _: JClass, ptr: jlong) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut OpdsApi);
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_isReadonly(_: JNIEnv, _: JClass, ptr: jlong) -> jboolean {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    match api.is_readonly() {
        Ok(readonly) => {
            if readonly {
                JNI_TRUE
            } else {
                JNI_FALSE
            }
        }
        Err(_) => JNI_FALSE,
    }
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorsNextCharByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&prefix)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.authors_next_char_by_prefix(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getSeriesNextCharByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&prefix)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.series_next_char_by_prefix(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorsByLastName(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&name)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.authors_by_last_name(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getSeriesBySerieName(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&name)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.series_by_serie_name(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getSeriesByGenreId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.series_by_genre_id(id as u32)
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorsByGenreId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.authors_by_genre_id(id as u32)
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getBooksByGenreIdAndDate(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
    date: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };


    env.get_string(&date)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.books_by_genre_id_and_date(id as u32, arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getGenresByMeta(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&name)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.genres_by_meta(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}
