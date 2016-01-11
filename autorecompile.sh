#!/bin/sh

ls *.tex *.rs *.log | grep -v rapport.log | entr -d ./compile.sh
