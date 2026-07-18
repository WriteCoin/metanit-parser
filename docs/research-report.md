# Практико-исследовательское задание: Git, платформы коллаборации и CI/CD

## 1. Модель ветвления: Git Flow

### Выбор стратегии

Для данного проекта выбрана стратегия **Git Flow**, так как:
- Проект имеет версионированные релизы (v1.0.0, v1.1.0)
- Требуется чёткое разделение между активной разработкой и стабильными релизами
- В проекте участвуют несколько параллельных фич

### Структура веток

| Ветка | Назначение | Защита |
|-------|-----------|--------|
| `main` | Стабильный релиз | Да |
| `develop` | Интеграция фич | Да |
| `feature/*` | Новые возможности | Нет |
| `release/*` | Подготовка релиза | Нет |
| `hotfix/*` | Срочные исправления | Нет |

### Правила приёма изменений

1. Прямые пуши в `main` и `develop` запрещены
2. Все изменения вливаются только через Pull Request
3. Каждый PR требует прохождения CI (lint → test → build → security)
4. Каждый PR требует как минимум один code review
5. Feature-ветки создаются от `develop` и вливаются в `develop`
6. Release-ветки создаются от `develop`, вливаются в `main` и `develop`
7. Hotfix-ветки создаются от `main`, вливаются в `main` и `develop`

### Процесс релиза

1. Создание `release/vX.Y.Z` от `develop`
2. Обновление версии и CHANGELOG
3. Финальное тестирование
4. Merge в `main` + тег `vX.Y.Z`
5. Merge обратно в `develop`

### Риски

- Накладные расходы на поддержку двух долгоживущих веток
- Release-ветки могут жить слишком долго и расходиться с develop
- Больше merge-конфликтов по сравнению с GitHub Flow

## 2. Issue Tracking

Создано **12 issues** разных типов:

| # | Тип | Название | Метки |
|---|-----|---------|-------|
| 1 | `bug` | Cache expiration is never checked on read | `bug` |
| 2 | `docs` | Add comprehensive rustdoc documentation | `documentation` |
| 3 | `feature` | Add pagination metadata to PageSummary | `enhancement` |
| 4 | `bug` | Parser panics on empty HTML document | `bug` |
| 5 | `feature` | Add content_html field to Page model | `enhancement`, `feature` |
| 6 | `tests` | Add integration tests with mock HTTP server | `enhancement`, `tests` |
| 7 | `ci` | Add code coverage reporting with tarpaulin | `enhancement`, `ci` |
| 8 | `security` | Add GitHub secret scanning and SAST | `enhancement`, `security` |
| 9 | `tests` | Add property-based tests for models | `enhancement`, `tests` |
| 10 | `ci` | Add dependency caching to CI pipeline | `enhancement`, `ci` |
| 11 | `docs` | Add CONTRIBUTING quickstart example | `documentation`, `good first issue` |
| 12 | `research` | Compare GitHub Actions vs GitLab CI/CD | `documentation`, `enhancement` |

Каждая issue содержит:
- Описание задачи
- Критерии готовности (Acceptance Criteria)
- Соответствующие метки

## 3. Pull Requests

Создано **8 PR/MR**, каждый с описанием, ссылкой на issue и результатом проверки:

| # | Ветка | Issue | Статус |
|---|-------|-------|--------|
| 13 | `feature/content-html` | #5 | Review + fix after feedback |
| 14 | `fix/cache-expiration` | #1 | Review approved |
| 15 | `feature/pagination-metadata` | #3 | Review approved |
| 16 | `feature/integration-tests` | #6 | Review approved |
| 17 | `docs/rustdoc-api` | #2 | Review approved |
| 18 | `feature/ci-improvements` | #7, #10 | Review approved |
| 19 | `feature/gitlab-ci` | #12 | Review approved |
| 20 | `feature/security-scanning` | #8 | Review approved |

### Пример code review (PR #13)

Ревьюер оставил замечание:
> Consider refactoring `parse_content` and `parse_content_html` to share code

Автор исправил:
- Добавлена общая функция `parse_body_raw` с параметром-замыканием

## 4. Исследовательская часть: GitHub Actions vs GitLab CI/CD

### 4.1 Настройка CI/CD

| Параметр | GitHub Actions | GitLab CI/CD |
|----------|---------------|--------------|
| Файлов конфигурации | 2 (ci.yml + codeql.yml) | 1 (.gitlab-ci.yml) |
| Строк конфигурации | 56 + 25 | 46 |
| Шагов в пайплайне | 5 jobs, 19 steps | 4 stages, 9 commands |
| Кеширование | Сторонний action | Встроенный `cache:` |
| Артефакты | `upload-artifact@v4` | Встроенный `artifacts:` |

### 4.2 Code Review

Обе платформы предоставляют практически идентичный UX для ревью:
- Inline-комментарии в PR/MR
- Thread-обсуждения
- Проверка статуса CI в PR
- Возможность фикса и обновления PR

GitHub имеет более зрелый CLI (`gh`).

### 4.3 Пайплайн

- **GitHub Actions**: быстрее холодный старт (~15s vs ~20s)
- **GitLab CI/CD**: лучше встроенное кеширование
- Оба поддерживают параллельные джобы
- Логи: оба имеют веб-интерфейс со сворачиваемыми секциями

### 4.4 Безопасность

- **GitHub**: CodeQL (зрелый SAST для Rust), Dependabot, Secret scanning
- **GitLab**: SAST-шаблоны, Dependency Scanning, Secret Detection

### 4.5 Применимость

| Тип проекта | Рекомендация | Причина |
|-------------|-------------|---------|
| Учебный | GitHub | Большое сообщество, бесплатно для студентов |
| Исследовательский | Любая | Обе бесплатны для публичных репозиториев |
| Промышленный | GitHub | Более зрелый marketplace, интеграции |
| Enterprise (self-hosted) | GitLab | Лучшее self-hosted решение |

**Вывод:** Для данного проекта (open-source Rust библиотека) рекомендуется
**GitHub Actions** как де-факто стандарт для Rust-экосистемы.

## 5. Выводы

В ходе выполнения работы было продемонстрировано:

1. **Git Flow** как стратегия ветвления для версионированных проектов
2. **Полный lifecycle**: Issue → Branch → Code → PR → CI → Review → Merge → Release
3. **Командная работа**: симуляция автора и ревьюера с замечаниями и исправлениями
4. **Issue Tracking**: 12 issues разных типов с критериями готовности
5. **CI/CD**: Настроены GitHub Actions и GitLab CI/CD
6. **Исследование**: Сравнение двух платформ по 5 критериям
