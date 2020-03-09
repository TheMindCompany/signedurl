# SignedURL

Generate signed url request for remote storage.  Currently supports AWS S3 but more will come.

## TODO

 Remaining work for 0.1.* releases:

 - JSON response.
 - TTL settings to module.  

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
    signedurl [FLAGS] [OPTIONS] [method] [SUBCOMMAND]

FLAGS:
        --no-buckets    Don't allow bucket to change
    -g, --gen-key       Generate key's with UUIDv4
    -d, --daemon        Daemon mode
    -v, --verbose       Enable verbose logging
        --help          Prints help information

OPTIONS:
    -b, --bucket <bucket>      Bucket target for signature
    -k, --key <key>            Key path target. (ie: filename)
        --prefix <prefix>      Let util append filename to key prefix
    -r, --region <region>      Region target [env: AWS_DEFAULT_REGION=]  [default: us-east-1]
    -t, --timeout <timeout>    Duration URL is invalid
    -p, --port <port>          Daemeon mode port [env: SIGNEDURL_PORT=]  [default: 8080]
    -h, --host <host>          Daemeon mode host [env: SIGNEDURL_HOST=]  [default: 127.0.0.1]

ARGS:
    <method>    The type of method being requested for signing url [default: PUT]

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

Daemon mode runs SignedURL as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---

**Fully Customized Signed Url's Allowed**

```bash
signedurl --host 0.0.0.0 --port 8080 --daemon
```

**GET /create/{bucket}/{key-path:*}**

Response:  
```json
{
  "data": {
    "type": "create-signed-url",
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

**GET /read/{bucket}/{key-path:*}**

Response:  
```json
{
  "data": {
    "type": "read-signed-url",
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

**GET /delete/{bucket}/{key-path:*}**

Response:  
```json
{
  "data": {
    "type": "delete-signed-url",
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

**Strongly Defined Signed Url's Only**

```bash
signedurl --bucket my-bucket --no-buckets --prefix user/bin --gen-key --daemon
```

**GET /create**

```json
{
  "data": {
    "type": "create-signed-url",
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

**GET /read/{key-path:*}**

Response:  
```json
{
  "data": {
    "type": "read-signed-url",
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

**GET /delete/{key-path:*}**

Response:  
```json
{
  "data": {
    "type": "delete-signed-url",
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

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
