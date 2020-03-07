# signedurl

Generate signed url request for remote storage.  Currently supports AWS S3 but more will come.

## TODO

 Remaining work for 0.1.* releases:

 - AWS S3 signed url high level config (time, region, content-type, ...)
 - Configuration file setting for default high level values
 - Dameon initialization(http api)

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
    -r, --region <region>      Region target [default: us-east-1]
    -t, --timeout <timeout>    Duration URL is invalid

ARGS:
    <method>    The type of method being requested for signing url

SUBCOMMANDS:
    configuration    Configuration options
    help             Prints this message or the help of the given subcommand(s)
```

The only required params for a fully configured yaml file will be `method` and `key`.   Currently default parameters for bucket, timeout, etc are in the pipeline to be added.

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
