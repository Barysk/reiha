#! /bin/bash

MODE="release"

./target/$MODE/reiha \
	./test/input.txt \
	--theme 000000xFFFFFF \
	--linear \
	--resolution 1600x1200 \
	--numbering \
	--numbering-anchor br \
	--background ./test/bg.png linear fill \
	--preview
	# --font ~/.fonts/Gentium-7.000/Gentium-Regular.ttf \ # example
	# --mono-font ~/.fonts/MPlus/Mplus1Code-Regular.ttf \ # example


