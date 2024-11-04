# Spice AI Take-Home Assessment

## Purpose

This is very minimal REPL written in Rust that uses Spice's query federation to query information about Shakespeare plays from multiple sources: a MySQL database and a PostgreSQL database. Acceleration is also enabled, making the queries somewhat faster, though the difference would be more notable if the databases were running on a different machine.

Source for the Shakespeare data: [https://www.opensourceshakespeare.org/downloads/](https://www.opensourceshakespeare.org/downloads/)

## How to run

The databases used by the application are run inside Docker containers on the host. The commands below will take care of setting them up.

### Build

```bash
make build
```

### Run

1. Start the databases and the Spice runtime:

```bash
make start
```

2. Run the application

```bash
make run
```

To stop the containers and the runtime, run:

```bash
make stop
```

### Clean up

```bash
make clean
```

## Spicepods

The Spicepod for this project is simple: It just tells Spice about the different MySQL and PostgreSQL tables and how to connect to them.
