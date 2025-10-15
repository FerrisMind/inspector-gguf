# Design Document

## Overview

Система автоматического обновления CHANGELOG и версионирования будет анализировать историю коммитов Git для определения типа и масштаба изменений, затем обновлять соответствующие файлы проекта согласно принципам семантического версионирования.

## Architecture

### Компоненты системы

1. **Git Commit Analyzer** - анализ истории коммитов
2. **Version Calculator** - определение новой версии по семантическому версионированию  
3. **CHANGELOG Updater** - обновление файла CHANGELOG.md
4. **Cargo.toml Updater** - обновление версии в конфигурации проекта

### Поток данных

```
Git History → Commit Analysis → Version Calculation → File Updates
```

## Components and Interfaces

### Git Commit Analyzer

**Назначение:** Извлечение и категоризация изменений из коммитов

**Входные данные:**
- История коммитов Git (git log)
- Текущая версия из Cargo.toml

**Выходные данные:**
- Список изменений по категориям (Added, Changed, Fixed)
- Тип изменений (major, minor, patch)

**Логика категоризации:**
- `feat:` или добавление новых файлов → Added (minor)
- `fix:` или исправления → Fixed (patch)  
- `refactor:`, `update:`, изменения существующих файлов → Changed (minor)
- `docs:` → Added/Changed в зависимости от контекста
- `BREAKING CHANGE` в сообщении → major изменение

### Version Calculator

**Назначение:** Определение новой версии согласно SemVer

**Входные данные:**
- Текущая версия (MAJOR.MINOR.PATCH)
- Тип изменений (major/minor/patch)

**Выходные данные:**
- Новая версия

**Логика версионирования:**
- Major: breaking changes → X+1.0.0
- Minor: новые функции → X.Y+1.0  
- Patch: только исправления → X.Y.Z+1

### CHANGELOG Updater

**Назначение:** Обновление структуры и содержимого CHANGELOG.md

**Операции:**
1. Парсинг существующего CHANGELOG
2. Извлечение содержимого секции [Unreleased]
3. Создание новой секции версии с датой
4. Перенос изменений из [Unreleased] в новую секцию
5. Создание новой пустой секции [Unreleased]

### Cargo.toml Updater

**Назначение:** Обновление версии в файле конфигурации проекта

**Операции:**
1. Чтение текущего Cargo.toml
2. Замена строки version = "старая_версия" на version = "новая_версия"
3. Сохранение обновленного файла

## Data Models

### CommitInfo
```rust
struct CommitInfo {
    hash: String,
    message: String,
    category: ChangeCategory,
    description: String,
}
```

### ChangeCategory
```rust
enum ChangeCategory {
    Added,    // Новые функции
    Changed,  // Изменения существующих функций
    Fixed,    // Исправления багов
}
```

### VersionChange
```rust
enum VersionChange {
    Major,    // Breaking changes
    Minor,    // Новые функции
    Patch,    // Исправления
}
```

### ChangelogSection
```rust
struct ChangelogSection {
    version: String,
    date: String,
    added: Vec<String>,
    changed: Vec<String>,
    fixed: Vec<String>,
}
```

## Error Handling

### Git Errors
- Отсутствие Git репозитория → Ошибка с инструкциями
- Недоступность истории коммитов → Предупреждение и ручной режим

### File Errors  
- Отсутствие CHANGELOG.md → Создание нового файла с базовой структурой
- Отсутствие Cargo.toml → Критическая ошибка (не Rust проект)
- Проблемы с правами доступа → Ошибка с инструкциями по исправлению

### Version Errors
- Некорректный формат версии → Попытка исправления или ошибка
- Конфликт версий → Предупреждение и запрос подтверждения

## Testing Strategy

### Unit Tests
- Тестирование парсинга коммитов с различными форматами сообщений
- Проверка логики определения типа изменений
- Валидация расчета новых версий
- Тестирование обновления файлов

### Integration Tests  
- Тестирование полного цикла обновления на тестовом репозитории
- Проверка сохранения форматирования CHANGELOG
- Валидация корректности обновления Cargo.toml

### Edge Cases
- Пустая история коммитов
- Коммиты без стандартных префиксов
- Множественные типы изменений в одном коммите
- Очень длинные сообщения коммитов