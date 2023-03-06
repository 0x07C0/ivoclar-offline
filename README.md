# Ivoclar offline demo

This program will scrape the `ids` page of ivoclar website, hydrate it with real data and serve it offline on localhost.

It uses a Selenium for hydration since data is only available after JS downloaded it.

## Try it out!

### Fast way

Simply run `make full` in your this project's directory.

### Manual way

1. Install [Rust](https://rustup.rs/)
2. Install [geckodriver](https://github.com/mozilla/geckodriver):
```bash
$ cargo install geckodriver
```
3. Install [serve-directory](https://gitlab.com/skubalj/serve-directory):
```bash
$ cargo install serve-directory
```
4. Run geckodriver:
```bash
$ geckodriver
```
5. In another terminal run:
```bash
$ wget -E -k -p -r -l 0 https://www.ivoclar.com/en_us/ids
$ mv www.ivoclar.com page
$ cargo run 
```
6. Finally run to serve the offline website:
```bash
$ serve-directory -l -p 8080 ./page
```
7. Open `http://127.0.0.1:8080/en_us/ids/`in your browser to view the result.
