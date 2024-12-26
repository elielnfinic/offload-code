# Offload-code 

This repository contains the code for the offloading of codebase that consume a lot of storage.
Some programming languages like Rust keep the external dependencies in the target folder which can consume a lot of storage. This codebase is an attempt to offload the dependencies to a different location.
In some cases, consumed storage can go from a few GBs to 15 - 20 GBs. This is a huge amount of storage for a single project.
For example, you can try to build `Substrate`, `Cairo`, and some other Rust projects.

## How to use
Run the following command to offload the dependencies to a different location.
```bash
offload-code --path <path-to-codebase>
```

## How to install
```bash
curl -s https://raw.githubusercontent.com/abhinavkorpal/offload-code/main/install.sh | bash
source ~/.bashrc
offload-code-up
```

## How to update
```bash
offload-code-up
```


