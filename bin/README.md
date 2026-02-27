<div align='center'>

# bis_rust - CLI приложение для чтения данных о транзакциях из двух файлов и сравнения.

</div>

---

## Технологии

Проект создан на базе:

-   Rust, Cargo

## Реализованный функционал

CLI приложение для чтения данных о транзакциях из двух файлов и их сравнения.

## Сборка проекта

cargo build

## Тестироание

cargo test -- --test-threads=1

## Запуск проекта

cargo run --bin comparer <filename1> <txt|csv|bin> <txt|csv|bin> <filename2>
