# Changelog

## [0.2.2] 2024-12-17

### Fixed

- wait for other plugins to generate certs

## [0.2.1] 2024-12-10

### Changed
- logging: added offending method name if verify_rune fails
- upgraded dependencies

## [0.2.0] 2024-09-26

### Added
- log rune id, method and params when access was granted

### Changed
- ``clnrest-cors-origins`` is now a multi option, meaning you have to specify it multiple times if you have multiple values, you must no longer feed it multiple values in a single string. This change was missing to be a drop-in replacement with clnrest.py

## [0.1.2] 2024-07-31

### Changed

- ``clnrest-swagger-root`` is now `"/"` again by default, but ``"/"`` will redirect to ``"/swagger-ui"`` while other values will enforce the custom path
- Also call ``plugin.shutdown()`` on a shutdown notification. This is probably not fixing 100% of the shutdown issues (maybe a cln-plugin bug)
- Add ``upgrade`` header to the tests. This is needed to differentiate between an intent to establish a websocket connection and an intent to open the swagger-ui on the same path. Usually websocket clients inlcude this header automatically but the tests were not
- Extracted release binaries are now called ``clnrest-rs``

## [0.1.0] 2024-07-27

### Added

- initial release