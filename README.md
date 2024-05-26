# Diesel ORM discovery

## Prerequisite

### Install diesel cli

```
cargo install diesel_cli --no-default-features --features postgres
```

### Install task runner

See https://taskfile.dev/installation/

```
sh -c "$(curl --location https://taskfile.dev/install.sh)" -- -d -b ~/.local/bin
```

### Prepare database

```
task rundb
task setupdb
```

## Run commands

The command `task --list-all` allows you to access the available actions.
