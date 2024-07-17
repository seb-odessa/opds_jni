package org.opds.api.tests;

import org.opds.api.models.*;
import org.opds.api.jni.Wrapper;

import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class Main {
    public static void main(String[] args) throws Exception {
        Wrapper wrapper = new Wrapper();
        Wrapper.OpdsApi api = wrapper.create("file:data/fb2-768381-769440.db?mode=ro");

        assert_true("api.isReadonly()", api.isReadonly());

        assert_eq("api.getAuthorsNextCharByPrefix()", api.getAuthorsNextCharByPrefix(""),
                Arrays.asList(
                        "А Б В Г Д Е Ё Ж З И Й К Л М Н О П Р С Т У Ф Х Ц Ч Ш Щ Э Ю Я д н ф D F K R S W"
                                .split("\\s+")));

        assert_eq("api.getSeriesNextCharByPrefix()", api.getSeriesNextCharByPrefix(""),
                Arrays.asList(
                        "А Б В Г Д З И К Л М Н О П Р С Т У Х Ц Ч Ш Э Ю Я п C E L N P 8"
                                .split("\\s+")));

        assert_eq("api.getAuthorsByLastName()", api.getAuthorsByLastName("Кейн"),
                List.of("Адель Кейн", "Рэйчел Кейн"));

        assert_eq("api.getSeriesBySerieName()", api.getSeriesBySerieName("Кровь на воздух"),
                List.of("Кровь на воздух [Павел Сергеевич Иевлев] (2)"));

        assert_eq("api.getSeriesByGenreId()", api.getSeriesByGenreId(24),
                List.of("Варяг [Мазин] [Александр Владимирович Мазин] (1)",
                        "Восток (РИПОЛ) [Владимир Вячеславович Малявин] (1)"));

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

        assert_eq("api.getBooksByGenreIdAndDate()", api.getBooksByGenreIdAndDate(24, "2024-06-0%"),
                List.of("Игра королев. Женщины, которые изменили историю Европы - Сара Гриствуд (2024-06-07) [2.67 MB]",
                        "Рыцари, закованные в сталь - Говард Пайл (2024-06-01) [2.46 MB]"));

        assert_eq("api.getGenresByMeta()", api.getGenresByMeta("Деловая литература"),
                List.of("Карьера, кадры",
                        "Маркетинг, PR",
                        "Финансы",
                        "Экономика"));

        assert_eq("api.getGenresByMeta()", api.getMetaGenres(),
                List.of("Деловая литература",
                        "Детективы и Триллеры",
                        "Документальная литература",
                        "Дом и семья",
                        "Драматургия",
                        "Искусство, Искусствоведение, Дизайн",
                        "Компьютеры и Интернет",
                        "Литература для детей",
                        "Любовные романы",
                        "Наука, Образование",
                        "Поэзия",
                        "Приключения",
                        "Проза",
                        "Прочее",
                        "Религия, духовность, Эзотерика",
                        "Справочная литература",
                        "Старинное",
                        "Техника",
                        "Учебники и пособия",
                        "Фантастика",
                        "Фольклор",
                        "Эзотерика",
                        "Юмор"));

        assert_eq("api.getAuthorsByBooksIds()",
                api.getAuthorsByBooksIds(new int[] { 768409, 768571, 768746, 768750 }),
                List.of("Анатолий Сергеевич Бернацкий",
                        "Сара Гриствуд",
                        "Александр Викторович Марков",
                        "Говард Пайл"));

        assert_eq("api.getSeriesByAuthorIds()",
                api.getSeriesByAuthorIds(50, 42, 281),
                List.of("Кровь на воздух [Павел Сергеевич Иевлев] (2)"));

        assert_eq("api.getAuthorByIds()",
                api.getAuthorByIds(50, 42, 281),
                "Павел Сергеевич Иевлев");

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

    public static <T> void assert_eq(String msg, Wrapper.Result<T> res, String rhv) throws Exception {
        System.out.print(msg);

        if (res.isSuccess()) {
            String lhv = res.getValue().toString();

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