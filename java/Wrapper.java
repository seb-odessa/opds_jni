import java.util.List;

public class Wrapper {
    static {
        System.loadLibrary("opds_jni"); // Загрузи библиотеку
    }

    public OpdsApi create(String dbPath) {
        return new OpdsApi(createOpdsApi(dbPath));
    }

    public static class OpdsApi {
        private long ptr = 0;

        private OpdsApi(long ptr) {
            this.ptr = ptr;
        }

        public void close() {
            Wrapper.destroyOpdsApi(this.ptr);
            this.ptr = 0;
        }

        public boolean isReadonly() {
            return Wrapper.isReadonly(this.ptr);
        }

        public Wrapper.Result<List<String>> getAuthorsNextCharByPrefix(String prefix) {
            return Wrapper.getAuthorsNextCharByPrefix(this.ptr, prefix);
        }

        public Wrapper.Result<List<String>> getSeriesNextCharByPrefix(String prefix) {
            return Wrapper.getSeriesNextCharByPrefix(this.ptr, prefix);
        }

        public Wrapper.Result<List<Author>> getAuthorsByLastName(String name) {
            return Wrapper.getAuthorsByLastName(this.ptr, name);
        }

        public Wrapper.Result<List<String>> getSeriesBySerieName(String name) {
            return Wrapper.getSeriesBySerieName(this.ptr, name);
        }

        public Wrapper.Result<List<String>> getSeriesByGenreId(int id) {
            return Wrapper.getSeriesByGenreId(this.ptr, id);
        }

        public Wrapper.Result<List<Author>> getAuthorsByGenreId(int id) {
            return Wrapper.getAuthorsByGenreId(this.ptr, id);
        }

        public Wrapper.Result<List<String>> getBooksByGenreIdAndDate(int id, String date) {
            return Wrapper.getBooksByGenreIdAndDate(this.ptr, id, date);
        }

        public Wrapper.Result<List<String>> getSeriesByAuthorIds(int fid, int mid, int lid) {
            return Wrapper.getSeriesByAuthorIds(this.ptr, fid, mid, lid);
        }

        public Wrapper.Result<List<Value>> getGenresByMeta(String name) {
            return Wrapper.getGenresByMeta(this.ptr, name);
        }
    }

    public static native long createOpdsApi(String dbPath);

    public static native void destroyOpdsApi(long api);

    public static native boolean isReadonly(long api);

    public static native Result<List<String>> getAuthorsNextCharByPrefix(long api, String prefix);

    public static native Result<List<String>> getSeriesNextCharByPrefix(long api, String prefix);

    public static native Result<List<Author>> getAuthorsByLastName(long api, String name);

    public static native Result<List<String>> getSeriesBySerieName(long api, String name);

    public static native Result<List<String>> getSeriesByGenreId(long api, int id);

    public static native Result<List<Author>> getAuthorsByGenreId(long api, int id);

    public static native Result<List<String>> getBooksByGenreIdAndDate(long api, int id, String date);

    public static native Result<List<String>> getSeriesByAuthorIds(long api, int fid, int mid, int lid);

    public static native Result<List<Value>> getGenresByMeta(long api, String name);

    public static class Result<T> {
        private final T value;
        private final String error;
        private final boolean isSuccess;

        private Result(T value, String error, boolean isSuccess) {
            this.value = value;
            this.error = error;
            this.isSuccess = isSuccess;
        }

        public static <T> Result<T> success(T value) {
            return new Result<>(value, null, true);
        }

        public static <T> Result<T> error(String error) {
            return new Result<>(null, error, false);
        }

        public boolean isSuccess() {
            return isSuccess;
        }

        public T getValue() {
            return value;
        }

        public String getError() {
            return error;
        }
    }
}
