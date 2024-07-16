use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use opds_api::OpdsApi;
use result::JavaObject;

mod result;

#[no_mangle]
pub extern "C" fn Java_Wrapper_createOpdsApi(mut env: JNIEnv, _: JClass, path: JString) -> jlong {
    let path: String = env
        .get_string(&path)
        .expect("Couldn't get java string!")
        .into();
    let api = OpdsApi::try_from(&path).expect("Failed to open database");
    Box::into_raw(Box::new(api)) as jlong
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_destroyOpdsApi(_: JNIEnv, _: JClass, ptr: jlong) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut OpdsApi);
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_isReadonly(_: JNIEnv, _: JClass, ptr: jlong) -> jboolean {
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
pub extern "C" fn Java_Wrapper_getAuthorsNextCharByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let prefix: String = env
        .get_string(&prefix)
        .expect("Couldn't get java string!")
        .into();

    match api.authors_next_char_by_prefix(&prefix) {
        Ok(list) => {
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getSeriesNextCharByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let prefix: String = env
        .get_string(&prefix)
        .expect("Couldn't get java string!")
        .into();

    match api.series_next_char_by_prefix(&prefix) {
        Ok(list) => {
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getAuthorsByLastName(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let name: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();

    match api.authors_by_last_name(&name) {
        Ok(list) => {
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getSeriesBySerieName(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let name: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();

    match api.series_by_serie_name(&name) {
        Ok(series) => {
            let list = series
                .into_iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<_>>();
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getSeriesByGenreId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    match api.series_by_genre_id(id as u32) {
        Ok(series) => {
            let list = series
                .into_iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<_>>();
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getAuthorsByGenreId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    // let res = api
    //     .authors_by_genre_id(id as u32)
    //     .and_then(|list| JavaObject::try_from((env, list)));

    // match res {
    //     Ok(obj) => obj.ptr,
    //     Err(err) =>
    // }

    match api.authors_by_genre_id(id as u32) {
        Ok(list) => {
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getBooksByGenreIdAndDate(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    id: jint,
    date: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let date: String = env
        .get_string(&date)
        .expect("Couldn't get java string!")
        .into();

    match api.books_by_genre_id_and_date(id as u32, date) {
        Ok(series) => {
            let list = series
                .into_iter()
                .map(|a| format!("{a}"))
                .collect::<Vec<_>>();
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_Wrapper_getGenresByMeta(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    name: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };
    let name: String = env
        .get_string(&name)
        .expect("Couldn't get java string!")
        .into();

    match api.genres_by_meta(&name) {
        Ok(list) => {
            JavaObject::try_from((env, list))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
        Err(err) => {
            JavaObject::try_from((&mut env, err))
                .inspect_err(|e| println!("{e}"))
                .unwrap()
                .ptr
        }
    }
}
