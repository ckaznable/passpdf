# passpdf

Remove password protection from PDF files.

## Install

```
cargo install --path --git https://github.com/ckaznable/passpdf.git
```

## Usage

```
passpdf <pdf-file>
```

Prompts for the password, then saves an unlocked copy as `<filename>_unlocked.pdf`.

```
$ passpdf secret.pdf
Password:
secret_unlocked.pdf
```
