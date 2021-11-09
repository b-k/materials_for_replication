go:
	docker run --rm -d -p 4444:4444 -p 5900:5900 --name selenium-server -v /dev/shm:/dev/shm selenium/standalone-firefox-debug:3.141.59-zinc
	cargo run get_jrnls
	cargo run get_tab
