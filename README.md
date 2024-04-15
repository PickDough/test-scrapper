## Scrapper of workflow runs
This repository can look at finished workflow runs and parse errors from job logs, generating a Github Pages display of the failed tests.

### Current Limitions 
* Repository can only search for failed **cucumber** tests runned by flutter
* Link to feature is not tested for more than one repository and may be broken for different contexts
  
## Usage
### Cli
 ```bash
  test-scrapper 
  --run-id=12345678
  --github-token=SECRET
  --owner_repo=PickDough/test-scrapper
  --output=./
```
The following command will output the gatherd failed test to specified path *(default - runs.json)*.
### Example Json
```json
  [
    {
      "run_id": 8394460535,
      "jobs": [
        {
          "job": "test (E2E, ios)",
          "failures": [
            {
              "scenario": {
                "name": "Scenario Member disables audio before call",
                "link": "https://github.com/instrumentisto/medea-jason/blob/master/e2e/tests/features/media_disable.feature"
              },
              "step": "And Alice's device video remote track from Bob is enabled # :0 took 30001"
            }
          ]
        }
      ]
    }
]
```
---
### Workflow Dispatch
You can trigger the whole process of scrapping logs as well as publishing the result on the Github Pages via call to workflow dispath.

```yml
  - name: curl
    run: curl -L 
              -X POST 
              -H "Accept:application/vnd.github+json"
              -H "Authorization:Bearer ${{ secrets.GITHUB_TOKEN }} " 
              -H "X-GitHub-Api-Version:2022-11-28" 
              https://api.github.com/repos/PickDough/test-scrapper/actions/workflows/post-rust.yml/dispatches 
              -d '{"ref":"${{ github.ref }}","inputs":{"run-id":"${{ github.run_id }}"}}'
```
