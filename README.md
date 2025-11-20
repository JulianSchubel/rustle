# Rustle  

A small command line ETL tool.

![](assets/images/logo_alpha.png)

# Quickstart

# Commands  

<details>
<summary>Bootstrap</summary>
</details>

<details>
<summary>Generate</summary>

<details open>

<summary>Usage</summary>  

```bash
rustle generate <output_path> --rows <N> --format <csv|ndjson>
```

</details> 

The `rustle generate` command creates synthetic datasets for testing the ETL
pipeline. It can generate CSV or NDJSON files containing realistic `metrics`
records.

| Field       | Type   | Description                         |
| ----------- | ------ | ----------------------------------- |
| `id`        | string | Unique identifier (UUID v4)         |
| `timestamp` | string | ISO-8601 timestamp                  |
| `value`     | float  | Random sensor measurement           |
| `tag`       | string | Random label                        |

<details> 

<summary>Required Arguments</summary>

| Argument        | Description                           |
| --------------- | ------------------------------------- |
| `<output_path>` | File to write the generated data into |

</details>

<details> 

<summary>Options</summary>

| Flag              | Description                                              |
| ----------------- | -------------------------------------------------------- |
| `--records <N>`      | Number of synthetic records to generate (default: `100000`) |
| `--format <csv\| ndjson>` | Output format (default: `csv`) |                 |
| `--seed <number>` | Optional RNG seed for reproducible output                |

</details>


</details>

<details>

<summary>Run</summary>        
    
</details> 
