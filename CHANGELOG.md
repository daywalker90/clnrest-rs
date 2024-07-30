# Changelog

## [0.1.1] 2024-07-30

### Changed

- ``clnrest-swagger-root`` is now "/" again by default, but "/" will redirect to "/swagger-ui" while other values will enforce the custom path
- also call ``plugin.shutdown()`` on shutdown event but probably not fixed 100% of the shutdown issues (maybe a cln-plugin bug)
- add ``upgrade`` header to the tests. This is needed to differentiate between an intent to establish a websocket connection and an intent to open the swagger-ui on the same path. Usually websocket client inlcude this header automatically but the tests were not
- extracted release binaries are now called ``clnrest-rs``

## [0.1.0] 2024-07-27

### Added

- initial release