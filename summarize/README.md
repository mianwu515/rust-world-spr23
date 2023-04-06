# A Rust Summarize Service with OpenAI API 

The service has been deployed to duke virtual machine. Access it from [here](http://vcm-30756.vm.duke.edu:8989/)!

![UI](summarize.png)

### Usage
- Get your development API Key from [here](https://platform.openai.com/account/api-keys)

- Set it as the value of "OPENAI_API_KEY" in your local environment as an exported variable. e.g.,
```bash
export OPENAI_API_KEY="xxx"
```

- Run the program
```bash
cargo run
```
- Launch http://127.0.0.1:8080 in the browser

### Containerize
- Create a Dockerfile and deploy the service to AWS APP Runner. See instructions from [here](https://github.com/nogibjj/rust-world-spr23/tree/main/actix-containerized-microservice-wk3/actixdocker#this-is-to-build-an-image-out-of-the-dockerfile).
    - Build container out of the Docker image: run `make build --build-arg OPENAI_API_KEY="xxx"`
    - Clean build: `docker build --no-cache --build-arg OPENAI_API_KEY="xxx" -t summarize .` or run `make build-no-cache --build-arg OPENAI_API_KEY="xxx"`

### References
- [OpenAI-examples](https://platform.openai.com/examples)
- [OpenAI-Summarize-playground](https://platform.openai.com/playground/p/default-summarize?model=text-davinci-003)


### Appendix - troubleshoots of the deployment on a Linux machine
- check whether a port has been used: `sudo lsof -i :9898 -sTCP:LISTEN`

- "error: failed to run custom build command for `openssl-sys v0.9.84`"
    - run `sudo apt-get update`
    - `sudo apt-get install pkg-config libssl-dev`
    - `export OPENSSL_DIR=/usr/local/ssl`

- find / -type d -name "openssl" 2>/dev/null

-- Output:

```bash
> /usr/include/x86_64-linux-gnu/openssl
> /usr/include/openssl
> /usr/lib/python3/dist-packages/cryptography/hazmat/backends/openssl
> /usr/lib/python3/dist-packages/cryptography/hazmat/bindings/openssl
> /usr/share/doc/openssl
```

-- Run
```bash
$ export OPENSSL_INCLUDE_DIR=/usr/include/openssl
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
export OPENSSL_DIR=/usr
```

-- Re-run `cargo run`
