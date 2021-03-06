# SignedURL

Generate signed url request for remote storage.  Currently supports AWS S3 but more will come.  Use as a cli based utility for daily engineering or as HTTP REST service.

## Install

Use the below command to install binary or build from source.

Binary install:  

```bash
curl https://themindcompany.github.io/signedurl/install.sh -sS | bash -s
```

Source install:

```bash
git clone https://github.com/TheMindCompany/signedurl.git
cd signedurl
make build
make install
```

### Configuration

In an effort to make configuration of `signedurl` simple we have defined a YAML config to handle credentials if not already confiogured.  Credentials will first look for service credentials on system and fall back to this `config.yaml` file when not found. This config can managed through `~/.signedurl/config.yaml`.  To change the base path you can set it with an environment field `CONFIG_SIGNEDURL_BASE`.

The first time you run the cli utility you will be prompted to configure existing credentials are not found.  

**~/.signedurl/config.yaml**  
```yaml
kind: config
version: alpha/1.0
specs:
  aws:
    key: ***
    secret: ***
```

### Autocompletion

For convenience purposes autocompletion scripts have been provided for most major shell programs.  Hopefully this make it more useable for daily use if engineers.

More information for each completion script provided:

```bash
signedurl configuration --help
```

## USAGE

Refer to the help menu for details `-h` or `--help`.

```bash
USAGE:
    signedurl [FLAGS] [OPTIONS] [method] [SUBCOMMAND]

FLAGS:
        --no-buckets    Don't allow bucket to change
    -g, --gen-key       Generate key's with UUIDv4
    -d, --daemon        Daemon mode
    -v, --verbose       Enable verbose logging
    -h, --help          Prints help information

OPTIONS:
    -b, --bucket <bucket>      Bucket target for signature
    -k, --key <key>            Key path target. (ie: filename)
        --prefix <prefix>      Let util append filename to key prefix
    -r, --region <region>      Region target [env: AWS_DEFAULT_REGION=]  [default: us-east-1]
    -t, --timeout <timeout>    Duration URL is invalid in milliseconds [default: 60000]
        --port <port>          Daemon mode port [env: SIGNEDURL_PORT=]  [default: 8080]
        --host <host>          Daemon mode host [env: SIGNEDURL_HOST=]  [default: 127.0.0.1]

ARGS:
    <method>    The type of method being requested for signing url [default: PUT]

SUBCOMMANDS:
    configuration    Configuration options
    help             Prints this message or the help of the given subcommand(s)
```

### Using the CLI

```bash
signedurl PUT --bucket my-bucket --key path/to/file.txt
```

```bash
signedurl GET --bucket my-bucket --key path/to/file.txt
```

```bash
signedurl DEL --bucket my-bucket --key path/to/file.txt
```

### Using as Daemon

Daemon mode runs SignedURL as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---

#### Fully Customized Signed Url's Allowed

```bash
signedurl --host 0.0.0.0 --port 8080 --daemon
```

##### GET /create/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "PUT",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /read/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "GET",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /delete/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "DELETE",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

---

#### Strongly Defined Signed Url's Only

```bash
signedurl --bucket my-bucket --no-buckets --prefix user/bin --gen-key --daemon
```

##### GET /create

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "PUT",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /read/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "GET",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /delete/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "DELETE",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```
___

Find more ways to configure it using `--help` flag. The above two use case examples show routes accessible when deployed with different parameters.


## Benchmark

Benchmark was made using most difficult route that with bottom level exceptions and generating UUIDv4 hash on localhost with a **single core**.

**Test System Spec**
- Macbook Pro
- 2.6 GHz 6-Core Intel Core i7
- 16 GB 2400 MHz DDR4
- Intel UHD Graphics 630 1536 MB

**1k @ 50 Concurrent Request (More realistic usage)**

```bash
ab -n 1000 -c 50 http://127.0.0.1:8080/create
```

```
Document Length:        425 bytes

Concurrency Level:      50
Time taken for tests:   0.393 seconds
Complete requests:      1000
Failed requests:        0
Total transferred:      534000 bytes
HTML transferred:       425000 bytes
Requests per second:    2542.56 [#/sec] (mean)
Time per request:       19.665 [ms] (mean)
Time per request:       0.393 [ms] (mean, across all concurrent requests)
Transfer rate:          1325.90 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   0.6      1       2
Processing:     1   14  15.0      4      64
Waiting:        0   14  15.0      4      64
Total:          2   15  15.2      5      65

Percentage of the requests served within a certain time (ms)
  50%      5
  66%     25
  75%     29
  80%     30
  90%     40
  95%     43
  98%     47
  99%     49
 100%     65 (longest request)
```

**10k @ 100 Concurrent Request(Pushing to extremes)**

```bash
ab -n 10000 -c 100 http://127.0.0.1:8080/create
```

```
Document Length:        425 bytes

Concurrency Level:      100
Time taken for tests:   3.735 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      5340000 bytes
HTML transferred:       4250000 bytes
Requests per second:    2677.60 [#/sec] (mean)
Time per request:       37.347 [ms] (mean)
Time per request:       0.373 [ms] (mean, across all concurrent requests)
Transfer rate:          1396.33 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    3   2.9      2      17
Processing:     1   34  36.2     21     219
Waiting:        1   33  36.1     20     219
Total:          2   37  36.0     25     220

Percentage of the requests served within a certain time (ms)
  50%     25
  66%     40
  75%     49
  80%     57
  90%     92
  95%    124
  98%    144
  99%    152
 100%    220 (longest request)
```

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
