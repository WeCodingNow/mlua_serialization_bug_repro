MODE ?= debug

grab-so:
	cp "target/${MODE}/librepro.so" repro.so
