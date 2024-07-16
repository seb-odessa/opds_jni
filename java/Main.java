import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) throws Exception {
        Wrapper wrapper = new Wrapper();
        Wrapper.OpdsApi api = wrapper.create("file:data/fb2-768381-769440.db?mode=ro");

        assert_true("api.isReadonly()", api.isReadonly());
        {
            Wrapper.Result<List<String>> result = api.getAuthorsNextCharByPrefix("");
            assert_true("result.isSuccess()", result.isSuccess());
            assert_eq("api.getAuthorsNextCharByPrefix()", String.join(" ", result.getValue()),
                    "А Б В Г Д Е Ё Ж З И Й К Л М Н О П Р С Т У Ф Х Ц Ч Ш Щ Э Ю Я д н ф D F K R S W");
        }
        {
            Wrapper.Result<List<String>> result = api.getSeriesNextCharByPrefix("");
            assert_true("result.isSuccess()", result.isSuccess());
            assert_eq("api.getSeriesNextCharByPrefix()", String.join(" ", result.getValue()),
                    "А Б В Г Д З И К Л М Н О П Р С Т У Х Ц Ч Ш Э Ю Я п C E L N P 8");
        }

        assert_eq("api.getAuthorsByLastName()", api.getAuthorsByLastName("Кейн"),
                List.of("Адель Кейн", "Рэйчел Кейн"));

        {
            Wrapper.Result<List<String>> result = api.getSeriesBySerieName("Кровь на воздух");
            assert_true("result.isSuccess()", result.isSuccess());
            assert_eq("api.getSeriesBySerieName()", result.getValue(),
                    List.of("Кровь на воздух [Павел Сергеевич Иевлев] (2)"));
        }
        {
            Wrapper.Result<List<String>> result = api.getSeriesByGenreId(24);
            assert_true("result.isSuccess()", result.isSuccess());
            assert_eq("api.getSeriesByGenreId()", result.getValue(),
                    List.of("Варяг [Мазин] [Александр Владимирович Мазин] (1)",
                            "Восток (РИПОЛ) [Владимир Вячеславович Малявин] (1)"));
        }

        assert_eq("api.getAuthorsByGenreId()", api.getAuthorsByGenreId(24),
                List.of("Дмитрий Михайлович Балашов",
                        "Анатолий Сергеевич Бернацкий",
                        "Александр Владимирович Волков",
                        "Сергей Михайлович Голицын",
                        "Сара Гриствуд",
                        "Александр Владимирович Мазин",
                        "Владимир Вячеславович Малявин",
                        "Александр Викторович Марков",
                        "Лев Карлосович Масиель Санчес",
                        "Говард Пайл",
                        "Джеймс Перкинс",
                        "Джордж Сартон",
                        "Евгений Викторович Старшов",
                        "Дон Холлуэй",
                        "Петер Шрайнер"));

        {
            Wrapper.Result<List<String>> result = api.getBooksByGenreIdAndDate(24, "2024-06-0%");
            assert_true("result.isSuccess()", result.isSuccess());
            assert_eq("api.getBooksByGenreIdAndDate()", result.getValue(),
                    List.of("Игра королев. Женщины, которые изменили историю Европы - Сара Гриствуд (2024-06-07) [2.67 MB]",
                            "Рыцари, закованные в сталь - Говард Пайл (2024-06-01) [2.46 MB]"));
        }
        {
            // Wrapper.Result<List<Value>> result = api.getGenresByMeta("Деловая
            // литература");
            // assert_true("result.isSuccess()", result.isSuccess());

            // assert_eq("api.getGenresByMeta()", result.getValue(),
            // List.of(new Value(47, "Карьера, кадры"),
            // new Value(44, "Маркетинг, PR"),
            // new Value(48, "Финансы"),
            // new Value(120, "Экономика")));

            assert_eq("api.getGenresByMeta()", api.getGenresByMeta("Деловая литература"),
                    List.of("Карьера, кадры",
                            "Маркетинг, PR",
                            "Финансы",
                            "Экономика"));
        }
        api.close();
    }

    public static void assert_true(String msg, boolean lhv) throws Exception {
        System.out.print(msg);

        if (lhv != true) {
            System.out.println("...Failed.");
            throw new Exception(lhv + " is not TRUE ");
        }
        System.out.println("...Ok");
    }

    public static <T> void assert_eq(String msg, T lhv, T rhv) throws Exception {
        System.out.print(msg);

        if (lhv.equals(rhv)) {
            System.out.println("...Ok");
        } else {
            System.out.println("...Failed.");
            throw new Exception("\n  left: '" + lhv + "'\n not EQ\n right: '" + rhv + "'");
        }
    }

    public static <T> void assert_eq(String msg, Wrapper.Result<List<T>> res, List<String> rhv) throws Exception {
        System.out.print(msg);

        if (res.isSuccess()) {
            List<String> lhv = res.getValue().stream()
                    .map(T::toString)
                    .collect(Collectors.toList());

            if (lhv.equals(rhv)) {
                System.out.println("...Ok");
            } else {
                System.out.println("...Failed.");
                throw new Exception("\n  left: '" + lhv + "'\n not EQ\n right: '" + rhv + "'");
            }

        } else {
            System.out.println("...Failed.");
            throw new Exception(res.getError());
        }

    }
}