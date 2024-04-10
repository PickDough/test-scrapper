* **test (medea-jason, android)**
  * Failure step 
    * Test on target Android API version
    * make test.flutter
  * Log
    * Terminate Emulator 
    * Error: Timeout waiting for emulator to boot.
* **test (E2E, ios)**
  * Failure step
    * SFU
    * make test.e2e.native
  * Log
    * tearDownAll() **?**
    * DriverError: Failed to fulfill RequestData due to remote error
    * Original error: ext.flutter.driver: (112) Service has disappeared
* **test (E2E, linux)**
    * Failure step 
      * Test on target Android API version
      * make test.flutter
    * Log
      * DriverError: Failed to fulfill RequestData due to remote error
      * Original error: ext.flutter.driver: (112) Service has disappeared
* **weird quay.io**
  * Timeout **?**


*TODO:*
  * [X] Download logs for each failed job
  * [X] Parse logs and capture Cucumber errors
  * [ ] Publish failed jobs to Github Pages
  * [ ] Query statistic for failed jobs

## Log Statistics Proposal 
* `list`
  * Failed Scenario `counter`
    * `list` 
      * Failed job `string`
    * `list`
      * Failed step `counter`

