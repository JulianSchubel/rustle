# Rustle  

A small command line ETL tool.

![](assets/images/logo_alpha.png)

# Quickstart

Install Rustle by building it from source:

```bash
cargo build --release
```

The binary executable `rustle` will then be located in the /target/release
directory. Copy the executable into the current working directory or navigate into the directory.

Rustle provides database bootstrapping for SQLite as part of the tool. To
bootstrap the database run

```bash
./rustle bootstrap metrics.db
```

Once complete generate a sample dataset as follows.

```bash
./rustle generate sample.csv --records 100000 --format csv
```

Once the sample data has been created, run the ETL pipeline 

```bash
./rustle run sample.csv metrics.db --threads 4 --buffer-size 10000 --batch-size 1000
```

There is a `unique key` constraint on the `id` field, so one must either 
generate new sample data or re-bootstrap the database after a successful run.
This can be done by providing the `--drop` option to the `generate` command.

# Notes on ETL strategy

The ETL strategy taken by Rustle is as follows:
- ETL operations are distributed over threads. That is, reading, transforming, and loading, are handled separately. 
- Threads communicate via MPMC bounded channels that ensure controlled memory usage. 
- Writes use prepared, cached, multi-row INSERT statements for high performance.

# Commands  

<details> <summary>Bootstrap</summary>
The rustle bootstrap command initializes the SQLite database by creating the
metrics table. One can drop an existing table when run with the `--drop` option.

```bash
    rustle bootstrap <db_path> [--drop] [--force]
```

<details> <summary>Required Arguments</summary>

| Argument    | Description                      |
| ----------- | -------------------------------- |
| `db_path` | Path to the SQLite database file |

</details> <details> <summary>Options</summary>

| Flag      | Description                                      |
| --------- | ------------------------------------------------ |
| `--drop`  | Drop existing tables before recreating them      |
| `--force` | Skip the confirmation prompt when using `--drop` |

</details>

</details>


<details open> <summary>Usage</summary>
</details>

<details> <summary>Generate</summary>

The `rustle generate` command creates synthetic datasets for testing the ETL
pipeline. It can generate `csv` or `ndjson` files containing realistic `metrics`
records.

Each generated record has the following structure:

| Field       | Type   | Description                 |
| ----------- | ------ | --------------------------- |
| `id`        | string | Unique identifier (UUID v4) |
| `timestamp` | string | ISO-8601 timestamp          |
| `value`     | float  | Random sensor measurement   |
| `tag`       | string | Random categorical label    |


<details open> <summary>Usage</summary>  

```bash
rustle generate <output_path> --rows <N> --format <csv|ndjson>
```

</details> 


| Field       | Type   | Description                         |
| ----------- | ------ | ----------------------------------- |
| `id`        | string | Unique identifier (UUID v4)         |
| `timestamp` | string | ISO-8601 timestamp                  |
| `value`     | float  | Random sensor measurement           |
| `tag`       | string | Random label                        |

<details> <summary>Required Arguments</summary>

| Argument        | Description                           |
| --------------- | ------------------------------------- |
| `<output>` | File path to write the generated data into |

</details>

<details> <summary>Options</summary>

| Flag              | Description                                              |
| ----------------- | -------------------------------------------------------- |
| `--records <N>`      | Number of synthetic records to generate (default: `100000`) |
| `--format <csv\| ndjson>` | Output format (default: `csv`) |                 |

</details>

</details>

<details> <summary>Run</summary>        

Runs the complete ETL pipeline: extract → transform → load.

Supports `csv` and `ndjson` input, multi-threaded transforms, and batched SQLite inserts.

<details open> <summary>Usage</summary>

```bash
./rustle run <input> <db_path> --threads <N> --buffer <size> --batch-size <size> 
```

</details> 

<details> <summary>Required Arguments</summary>
    
| Argument    | Description                      |
| ----------- | -------------------------------- |
| `<input>`   | Input CSV or NDJSON file         |
| `<db_path>` | Destination SQLite database path |

</details> <details> <summary>Options</summary>

| Flag                  | Description                                              |
| --------------------- | -------------------------------------------------------- |
| `-t, --threads <N>`   | Number of transform worker threads (default: `4`)        |
| `-b, --buffer <size>` | Bounded channel buffer size (default: `10000`)           |
| `--csv-headers`       | Whether the CSV input contains headers (default: `true`) |
| `--batch-size <size>` | Number of rows per SQLite insert batch (default: `1000`) |

</details>

</details> 
