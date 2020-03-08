# SignedURL

Generate signed url request for remote storage.  Currently supports AWS S3 but more will come.

## TODO

 Remaining work for 0.1.* releases:

 - AWS S3 signed url high level config (time, region, content-type, ...)
 - Configuration file setting for default high level values
 - Daemon routes.

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

## USAGE

Refer to the help menu for details `-h` or `--help`.

```bash
USAGE:
    signedurl [FLAGS] [OPTIONS] <method> --key <key> [SUBCOMMAND]

FLAGS:
    -v, --verbose    Enable verbose logging
    -h, --help       Prints help information

OPTIONS:
    -b, --bucket <bucket>      Bucket target for signature
    -k, --key <key>            Key path target. (ie: filename)
    -r, --region <region>      Region target [env: AWS_DEFAULT_REGION=]  [default: us-east-1]
    -t, --timeout <timeout>    Duration URL is invalid
    -d, --daemon <daemon>      Daemon mode
    -p, --port <port>          Daemeon mode [env: SIGNEDURL_PORT=]  [default: 8080]

ARGS:
    <method>    The type of method being requested for signing url

SUBCOMMANDS:
    configuration    Configuration options
    help             Prints this message or the help of the given subcommand(s)
```

The only required params for a fully configured yaml file will be `method` and `key`.   Currently default parameters for bucket, timeout, etc are in the pipeline to be added.

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

Daemon mode runs SignedURL as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents.

```bash
signedurl --bucket my-bucket --port 443 --daemon
```

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
