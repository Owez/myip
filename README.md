# myip

Simple ip address fetching, designed for scripts

- **[Landing Page](https://myip.ogriffiths.com)**

## Under-the-hood

This program uses Rust for the user-installed binary which hooks to curl based locally on a user's computer. The server it connects to is found in the `server/` directory which is constantly online.

## Options

```none
Usage: myip [OPTIONS]

  Simple ip address fetching, designed for scripts

Options:
    --help, -h      Shows this message
    --verbose, -v   Enables verbose mode for errors
```
