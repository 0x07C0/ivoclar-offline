full: rustup gecko serve hydrate open

run: hydrate open

hydrate: move
	geckodriver &
	cargo run

move: clean wget-ids wget-work
	mv www.ivoclar.com page

clean:
	rm -rf ./page

wget-ids:
	wget -E -k -p -r -l 0 https://www.ivoclar.com/en_us/ids || echo "Done"

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

gecko:
	cargo install geckodriver

serve:
	cargo install serve-directory

open:
	open http://127.0.0.1:8080/en_us/ids/
	serve-directory -l -p 8080 ./page