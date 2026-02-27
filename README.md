<div align='center'>

# bis_rust

</div>

Реализация банковской системы с использованием Rust

Проект создан в учебных целях

---

## Технологии

Проект создан на базе:

-   Rust, Cargo

## Реализованный функционал

Библиотека (crate) для парсинга/сериализации/десериализации финансовых данных в несколько форматов и отдельный исполняемый cli (консольное приложение) crate, использующий данную библиотеку.
Поддерживаемые форматы:
YPBankCsv — таблица банковских операций.
YPBankText — текстовый формат описания списка операций.
YPBankBin — бинарное предоставление списка операций.

Библиотека, обеспечивающая парсинг и сериализацию форматов.

Converter - консольное приложение для парсинга файлов и преобразования форматов.

Comparer - CLI приложение для чтения данных о транзакциях из двух файлов и сравнения.

## Сборка проекта

cargo build

## Тесты

cargo test -- --test-threads=1

## Запуск проекта

cargo run --bin convert <input-filename> <txt|csv|bin> <txt|csv|bin> <output-filename>
cargo run --bin comparer <filename1> <txt|csv|bin> <txt|csv|bin> <filename2>

## Примеры файлов в форматах txt, csv, bin

[Формат TXT](src/example/records_example.txt)

[Формат CSV](src/example/records_example.csv)

[Формат BIN](src/example/records_example.bin)


