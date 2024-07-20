use jni::objects::{JClass, JIntArray, JString};
use jni::sys::{jboolean, jint, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use opds_api::OpdsApi;
use result::JavaObject;

mod result;

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_createOpdsApi(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) -> jlong {
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
pub extern "C" fn Java_org_opds_api_jni_Wrapper_isReadonly(
    _: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jboolean {
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
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getSeriesByAuthorIds(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    fid: jint,
    mid: jint,
    lid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.series_by_author_ids(fid as u32, mid as u32, lid as u32)
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
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorByIds(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    fid: jint,
    mid: jint,
    lid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.author_by_ids(fid as u32, mid as u32, lid as u32)
        .and_then(|author| JavaObject::try_from((&mut env, author)))
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

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getMetaGenres(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.meta_genres()
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

fn to_vec_u32(env: &mut JNIEnv, ints: JIntArray) -> anyhow::Result<Vec<u32>> {
    let length = env.get_array_length(&ints)?;

    let mut ids = vec![0; length as usize];

    env.get_int_array_region(ints, 0, &mut ids)?;

    Ok(ids.into_iter().map(|x| x as u32).collect())
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorsByBooksIds(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    ids: JIntArray,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    to_vec_u32(&mut env, ids)
        .and_then(|ids| api.authors_by_books_ids(ids))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getBooksByAuthorIds(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    fid: jint,
    mid: jint,
    lid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.books_by_author_ids(fid as u32, mid as u32, lid as u32)
        .and_then(|author| JavaObject::try_from((&mut env, author)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getBooksByAuthorIdsWithoutSerie(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    fid: jint,
    mid: jint,
    lid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.books_by_author_ids_without_serie(fid as u32, mid as u32, lid as u32)
        .and_then(|author| JavaObject::try_from((&mut env, author)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getBooksByAuthorIdsAndSerieId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    fid: jint,
    mid: jint,
    lid: jint,
    sid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.books_by_author_ids_and_serie_id(fid as u32, mid as u32, lid as u32, sid as u32)
        .and_then(|author| JavaObject::try_from((&mut env, author)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getBooksBySerieId(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    sid: jint,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    api.books_by_serie_id(sid as u32)
        .and_then(|author| JavaObject::try_from((&mut env, author)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getAuthorsByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&prefix)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.search_authors_by_prefix(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}

#[no_mangle]
pub extern "C" fn Java_org_opds_api_jni_Wrapper_getSeriesByPrefix(
    mut env: JNIEnv,
    _: JClass,
    ptr: jlong,
    prefix: JString,
) -> jobject {
    let api: &OpdsApi = unsafe { &*(ptr as *const OpdsApi) };

    env.get_string(&prefix)
        .map_err(|e| anyhow::anyhow!("{e}"))
        .and_then(|str| Ok(Into::<String>::into(str)))
        .and_then(|arg| api.search_series_by_prefix(&arg))
        .and_then(|list| JavaObject::try_from((&mut env, list)))
        .unwrap_or_else(|err| JavaObject::try_from((&mut env, err)).unwrap())
        .ptr
}
