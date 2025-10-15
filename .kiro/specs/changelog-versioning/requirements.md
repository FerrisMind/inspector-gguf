# Requirements Document

## Introduction

Автоматизация процесса обновления CHANGELOG и версионирования проекта на основе истории коммитов Git с применением принципов семантического версионирования.

## Glossary

- **CHANGELOG**: Файл, содержащий хронологический список изменений для каждой версии проекта
- **Semantic Versioning**: Система версионирования в формате MAJOR.MINOR.PATCH
- **Git Commits**: История изменений в репозитории Git
- **Inspector GGUF**: Основная система для инспекции GGUF файлов
- **Cargo.toml**: Файл конфигурации Rust проекта, содержащий метаданные включая версию

## Requirements

### Requirement 1

**User Story:** Как разработчик, я хочу автоматически обновлять CHANGELOG на основе коммитов, чтобы поддерживать актуальную документацию изменений

#### Acceptance Criteria

1. WHEN анализируются коммиты Git, THE Inspector GGUF SHALL извлекать информацию о типах изменений из сообщений коммитов
2. WHEN обнаруживаются новые функции, THE Inspector GGUF SHALL добавлять их в секцию "Added" CHANGELOG
3. WHEN обнаруживаются исправления, THE Inspector GGUF SHALL добавлять их в секцию "Fixed" CHANGELOG
4. WHEN обнаруживаются изменения существующей функциональности, THE Inspector GGUF SHALL добавлять их в секцию "Changed" CHANGELOG
5. THE Inspector GGUF SHALL сохранять хронологический порядок изменений в CHANGELOG

### Requirement 2

**User Story:** Как разработчик, я хочу автоматически определять новую версию по семантическому версионированию, чтобы корректно отражать масштаб изменений

#### Acceptance Criteria

1. WHEN обнаруживаются breaking changes, THE Inspector GGUF SHALL увеличивать MAJOR версию
2. WHEN добавляются новые функции без breaking changes, THE Inspector GGUF SHALL увеличивать MINOR версию  
3. WHEN вносятся только исправления багов, THE Inspector GGUF SHALL увеличивать PATCH версию
4. THE Inspector GGUF SHALL обновлять версию в файле Cargo.toml
5. THE Inspector GGUF SHALL создавать новую секцию версии в CHANGELOG с датой релиза

### Requirement 3

**User Story:** Как разработчик, я хочу сохранить существующую структуру и форматирование CHANGELOG, чтобы поддерживать консистентность документации

#### Acceptance Criteria

1. THE Inspector GGUF SHALL сохранять существующий формат CHANGELOG согласно Keep a Changelog
2. THE Inspector GGUF SHALL перемещать содержимое секции [Unreleased] в новую версию
3. THE Inspector GGUF SHALL создавать новую пустую секцию [Unreleased] для будущих изменений
4. THE Inspector GGUF SHALL сохранять все существующие секции документа (Release Notes, Development Milestones, etc.)
5. THE Inspector GGUF SHALL использовать правильный формат даты в формате YYYY-MM-DD