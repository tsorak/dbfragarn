# dbFragarn

Command-line tool for querying a MySQL database

## Usage

### Command-line

```sh
URL=mysql://USERNAME:PASSWORD@HOST/DATABASE cargo run
```

### Using .env file

#### .env

```txt
URL=mysql://USERNAME:PASSWORD@HOST/DATABASE
```

#### Shell

```sh
cargo run
```

## TODO

- [x] Load URL from .env file
- [x] More structured table output
- [ ] Commands history (Queries & Responses)
- [ ] Edit queries (Traverse cursor through entered characters)
- [ ] Multiline query support? (add ';' requirement)
- [ ] Colors?
- [ ] Add column types support (Dates)
