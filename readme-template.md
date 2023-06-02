# "{{ project-name }}" Policy

## Documentation
Check the [documentation](./.pdk/docs/TABLE_OF_CONTENTS.md) for reference of available features, ready-to-go example policies and more.

## Local development life cycle

This project includes a makefile and a docker compose to simplify testing your custom policy.

Flex 1.3.0 will automatically been downloaded by docker compose, and is expected that you will use that version or greater during development.

1. To start up Flex you'll need a registration.yaml file of local Flex instance in the directory [test/config](test/config). If you already have the registration file you can use that one. Otherwise, to complete the registration we recommend:
    1. Go to `Runtime Manager`
    2. Navigate to the `Flex Gateway` tab
    3. Click the `Add Gateway` button
    4. Select `Docker` as your OS and copy the registration command replacing `--connected=true` to `--connected=false`
2. Once you want to deploy your custom policy you'll need to run the command `make deploy` on the same directory this readme file is.
   This command will build your policy and copy the yaml file to the directory where Flex will search for the configuration.
3. Navigate to [test](test) folder and execute `docker compose up` to start the Flex instance and the backend.
4. Hit your API to see your policy in action. `curl http://127.0.0.1:8081/my/path -v`
5. Each time you make modifications to your policy, rerun `make deploy` to make those modifications impact in your running Flex instance.
