# Token Transfer Analytics

Сервис для анализа трансферов токенов с расчетом метрик по адресам.

## Функциональность
- Генерирует фейковые данные трансферов токенов
- Сохраняет данные в in-memory или clickhouse хранилище
- Рассчитывает метрики по адресам: общий объем, средние цены, максимальный баланс

## Настройка
Создайте `.env` файл:
```
CLICKHOUSE_URL=your_clickhouse_url
CLICKHOUSE_USER=your_user
CLICKHOUSE_PASSWORD=your_password
CLICKHOUSE_DB=your_database
```

## Запуск
```bash
cargo run
```

## Тестирование
```bash
cargo test
```
